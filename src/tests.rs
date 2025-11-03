use super::*;

#[test]
fn branch() {
    let repo = Repo {
        host: Host::GitHub,
        owner: "zachfedor".to_string(),
        name: "degit-rs-test-repo".to_string(),
        subdir: None,
        gitref: Some("dev".to_string()),
    };
    assert_eq!(parse("zachfedor/degit-rs-test-repo#dev").unwrap(), repo);
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn tag() {
    let repo = Repo {
        host: Host::GitHub,
        owner: "zachfedor".to_string(),
        name: "degit-rs-test-repo".to_string(),
        subdir: None,
        gitref: Some("v0.0.1".to_string()),
    };
    assert_eq!(parse("zachfedor/degit-rs-test-repo#v0.0.1").unwrap(), repo);
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn commit() {
    let repo = Repo {
        host: Host::GitHub,
        owner: "zachfedor".to_string(),
        name: "degit-rs-test-repo".to_string(),
        subdir: None,
        gitref: Some("d75ef1c".to_string()),
    };
    assert_eq!(parse("zachfedor/degit-rs-test-repo#d75ef1c").unwrap(), repo);
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn subdir() {
    let repo = Repo {
        host: Host::GitHub,
        owner: "zachfedor".to_string(),
        name: "degit-rs-test-repo".to_string(),
        subdir: Some("subdir".to_string()),
        gitref: None,
    };
    assert_eq!(parse("zachfedor/degit-rs-test-repo/subdir").unwrap(), repo);
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn subdir_and_gitref() {
    let repo = Repo {
        host: Host::GitHub,
        owner: "zachfedor".to_string(),
        name: "degit-rs-test-repo".to_string(),
        subdir: Some("subdir".to_string()),
        gitref: Some("dev".to_string()),
    };
    assert_eq!(
        parse("zachfedor/degit-rs-test-repo/subdir#dev").unwrap(),
        repo
    );
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn github() {
    let repo = Repo {
        host: Host::GitHub,
        owner: "octocat".to_string(),
        name: "Spoon-Knife".to_string(),
        subdir: None,
        gitref: None,
    };

    assert_eq!(parse("octocat/Spoon-Knife").unwrap(), repo);
    assert_eq!(parse("github:octocat/Spoon-Knife").unwrap(), repo);
    assert_eq!(
        parse("https://github.com/octocat/Spoon-Knife.git").unwrap(),
        repo
    );
    assert_eq!(
        parse("git@github.com:octocat/Spoon-Knife.git").unwrap(),
        repo
    );
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn bitbucket() {
    let repo = Repo {
        host: Host::BitBucket,
        owner: "Rich_Harris".to_string(),
        name: "degit-test-repo".to_string(),
        subdir: None,
        gitref: None,
    };

    assert_eq!(
        parse("https://bitbucket.org/Rich_Harris/degit-test-repo.git").unwrap(),
        repo
    );
    assert_eq!(
        parse("git@bitbucket.org:Rich_Harris/degit-test-repo.git").unwrap(),
        repo
    );
    assert_eq!(
        parse("bitbucket:Rich_Harris/degit-test-repo").unwrap(),
        repo
    );
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

#[test]
fn gitlab() {
    let repo = Repo {
        host: Host::GitLab("gitlab.com".to_string()),
        owner: "zachfedor".to_string(),
        name: "spoon-knife".to_string(),
        subdir: None,
        gitref: None,
    };

    assert_eq!(parse("gitlab:zachfedor/spoon-knife").unwrap(), repo);
    assert_eq!(
        parse("https://gitlab.com/zachfedor/spoon-knife.git").unwrap(),
        repo
    );
    assert_eq!(
        parse("git@gitlab.com:zachfedor/spoon-knife.git").unwrap(),
        repo
    );
    assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
}

// TODO: Add tests for self-hosted GitLab repositories
// #[test]
// fn gitlab_hosted() {
//     let repo = Repo {
//         host: Host::GitLab("gitlab.gnome.org".to_string()),
//         owner: "bilelmoussaoui".to_string(),
//         name: "gtk-rust-template".to_string(),
//         subdir: None,
//         gitref: None,
//     };

//     assert_eq!(
//         parse("https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template").unwrap(),
//         repo
//     );
//     assert_eq!(
//         parse("git@gitlab.gnome.org:bilelmoussaoui/gtk-rust-template.git").unwrap(),
//         repo
//     );
//     assert_eq!(download(repo, PathBuf::from("/tmp/tests")).unwrap(), ());
// }
