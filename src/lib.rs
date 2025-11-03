use colored::*;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{error::Error, fmt, path::PathBuf, process::Command};
use tar::Archive;

#[derive(Debug, PartialEq)]
enum Host {
    GitHub,
    GitLab(String),
    BitBucket,
}

#[derive(Debug, PartialEq)]
struct Repo {
    host: Host,
    owner: String,
    name: String,
    subdir: Option<String>,
    gitref: Option<String>,
}

impl Repo {
    pub fn url(&self) -> String {
        match &self.host {
            Host::GitHub => format!("https://github.com/{}/{}", self.owner, self.name),
            Host::GitLab(domain) => format!("https://{}/{}/{}", domain, self.owner, self.name),
            Host::BitBucket => format!("https://bitbucket.org/{}/{}", self.owner, self.name),
        }
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let owner = self.owner.bold().underline();
        let mut project = self.name.as_str().to_string();
        if let Some(subdir) = self.subdir.as_ref() {
            project = format!("{project}/{subdir}")
        }
        if let Some(gitref) = self.gitref.as_ref() {
            project = format!("{project}#{gitref}")
        }
        let project = project.red();
        let host = match self.host {
            Host::GitHub => "GitHub".blue(),
            Host::GitLab(_) => "GitLab".blue(),
            Host::BitBucket => "BitBucket".blue(),
        };
        write!(f, "{}/{} from {}", owner, project, host)
    }
}

pub fn degit(src: &str, dest: &str) {
    let repo = parse(src).unwrap();
    match download(repo, PathBuf::from(dest)) {
        Err(x) => println!("{}", x),
        _ => (),
    }
}

fn parse(src: &str) -> Result<Repo, Box<dyn Error>> {
    let re = Regex::new(
            r"^(?:(?:https://)?([^:/]+\.[^:/]+)/|git@([^:/]+)[:/]|([^/]+):)?([^/\s]+)/([^/\s#]+)((?:/[^/\s#]+)+)?(?:/)?(?:#(.+))?"
        ).unwrap();

    let captures = re
        .captures(src)
        .ok_or_else(|| format!("Could not parse src: {src}"))?;

    // Determine host from multiple formats that might match regex
    let host = if let Some(domain) = captures.get(1).or_else(|| captures.get(2)) {
        match domain.as_str() {
            "github.com" => Host::GitHub,
            "bitbucket.org" => Host::BitBucket,
            other => Host::GitLab(other.to_string()),
        }
    } else if let Some(shorthand) = captures.get(3) {
        match shorthand.as_str() {
            "github" | "gh" => Host::GitHub,
            "gitlab" | "gl" => Host::GitLab("gitlab.com".to_string()),
            "bitbucket" | "bb" => Host::BitBucket,
            other => Host::GitLab(other.to_string()),
        }
    } else {
        Host::GitHub
    };

    let res = Repo {
        host,
        owner: captures.get(4).unwrap().as_str().to_string(),
        name: captures
            .get(5)
            .unwrap()
            .as_str()
            .trim_end_matches(".git")
            .to_string(),
        subdir: captures
            .get(6)
            .map(|m| m.as_str().trim_start_matches('/').to_string()),
        gitref: captures.get(7).map(|m| m.as_str().to_string()),
    };
    return Ok(res);
}

fn download(repo: Repo, dest: PathBuf) -> Result<(), Box<dyn Error>> {
    let hash = get_hash(&repo)?;

    let url = match &repo.host {
        Host::GitHub => format!("{}/archive/{}.tar.gz", repo.url(), hash),
        Host::GitLab(_) => format!(
            "{}/-/archive/{}/{}-{}.tar.gz",
            repo.url(),
            hash,
            repo.name,
            hash
        ),
        Host::BitBucket => format!("{}/get/{}.tar.gz", repo.url(), hash),
    };
    // println!("{}", url);
    let client = reqwest::Client::new();

    let request = client.get(&url).send().unwrap();
    match request.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::UNAUTHORIZED => {
            Err("Could not find repository.")?;
        }
        s => Err(format!("Received response status: {:?}", s))?,
    };

    let total_size = request.content_length();

    let pb = match total_size {
        Some(x) => {
            let p = ProgressBar::new(x);
            p.set_style(ProgressStyle::default_bar()
                     .template("> {wide_msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                     .progress_chars("#>-"));
            p
        }
        None => {
            let p = ProgressBar::new_spinner();
            p
        }
    };

    println!("Downloading {} to {}", repo, dest.display());
    // println!("{:#?}", request.content_length());

    let tar = GzDecoder::new(pb.wrap_read(request));
    let mut archive = Archive::new(tar);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .filter_map(|mut entry| -> Option<Result<PathBuf, Box<dyn Error>>> {
            let path = match entry.path() {
                Ok(p) => p,
                Err(e) => return Some(Err(e.into())),
            };

            // Strip root directory (first component of tar archive)
            let path = match path.strip_prefix(path.components().next().unwrap()) {
                Ok(p) => p.to_owned(),
                Err(e) => return Some(Err(e.into())),
            };

            // If subdirectory is specified, filter and strip it
            let final_path = if let Some(subdir) = &repo.subdir {
                if path.starts_with(subdir) {
                    match path.strip_prefix(subdir) {
                        Ok(p) => p.to_owned(),
                        Err(e) => return Some(Err(e.into())),
                    }
                } else {
                    return None; // Skip files not in the subdirectory
                }
            } else {
                path
            };

            match entry.unpack(dest.join(&final_path)) {
                Ok(_) => Some(Ok(final_path)),
                Err(e) => Some(Err(e.into())),
            }
        })
        .filter_map(|e| e.ok())
        .for_each(|x| pb.set_message(&format!("{}", x.display())));

    // archive.unpack(dest).unwrap();
    pb.finish_with_message("Done...");
    Ok(())
}

pub fn validate_src(src: String) -> Result<(), String> {
    parse(&src).map(|_| ()).map_err(|x| x.to_string())
}

pub fn validate_dest(dest: String) -> Result<(), String> {
    let path = PathBuf::from(dest);
    if path.exists() {
        if path.is_dir() {
            let count = std::fs::read_dir(&path).map_err(|x| x.to_string())?.count();
            if count != 0 {
                Err(format!("Directory is not empty: {}", path.display()))?
            }
        } else {
            Err("Destination is not a directory.")?
        }
    }
    let mut realpath = {
        if path.is_relative() {
            let mut realpath = std::fs::canonicalize(std::path::Path::new(".")).unwrap();

            for c in path.components() {
                // println!("component: {:?}", c);
                match c {
                    std::path::Component::ParentDir => {
                        realpath = realpath.parent().unwrap().to_path_buf()
                    }
                    std::path::Component::Normal(c) => realpath.push(c),
                    _ => (),
                }
            }
            realpath
        } else {
            path
        }
    };
    while !realpath.exists() {
        realpath.pop();
    }
    if std::fs::metadata(&realpath)
        .unwrap()
        .permissions()
        .readonly()
    {
        Err("Directory is read-only.")?
    }
    // println!("realpath: {:?}", realpath);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum RefType {
    Head,
    Branch,
    Tag,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
struct GitRef {
    ref_type: RefType,
    name: String,
    hash: String,
}

fn fetch_refs(repo: &Repo) -> Result<Vec<GitRef>, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("ls-remote")
        .arg(&repo.url())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("could not fetch remote {}: {}", repo.url(), stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"refs/(\w+)/(.+)").unwrap();

    stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| {
            let parts: Vec<&str> = row.split('\t').collect();
            if parts.len() != 2 {
                return Err(format!("could not parse git ref: {row}").into());
            }

            let hash = parts[0].to_string();
            let git_ref = parts[1];

            if git_ref == "HEAD" {
                return Ok(GitRef {
                    ref_type: RefType::Head,
                    name: git_ref.to_string(),
                    hash,
                });
            }

            let captures = re
                .captures(git_ref)
                .ok_or_else(|| format!("could not parse {git_ref}"))?;

            let ref_type_str = captures.get(1).unwrap().as_str();
            let name = captures.get(2).unwrap().as_str().to_string();

            let ref_type = match ref_type_str {
                "heads" => RefType::Branch,
                "tags" => RefType::Tag,
                "refs" => RefType::Other("ref".to_string()),
                other => RefType::Other(other.to_string()),
            };

            Ok(GitRef {
                ref_type,
                name,
                hash,
            })
        })
        .collect()
}

fn get_hash(repo: &Repo) -> Result<String, Box<dyn Error>> {
    let refs = fetch_refs(repo)?;

    // If no gitref specified or it's "HEAD", return HEAD
    if repo.gitref.is_none() || repo.gitref.as_deref() == Some("HEAD") {
        if let Some(head_ref) = refs.iter().find(|r| r.ref_type == RefType::Head) {
            return Ok(head_ref.hash.clone());
        }
    }

    // Otherwise search for matching ref
    if let Some(gitref) = &repo.gitref {
        for r in &refs {
            if &r.name == gitref || r.hash.starts_with(gitref.as_str()) {
                return Ok(r.hash.clone());
            }
        }
    }

    Err("Reference not found.".into())
}

#[cfg(test)]
mod tests;
