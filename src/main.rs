use clap::{App, Arg, crate_version};

fn main() {
    let matches = App::new("degit-rs")
        .version(crate_version!())
        .about("Download the contents of a git repository without cloning it.")
        .arg(
            Arg::with_name("src")
                .help("the source repo you want to download")
                .long_help(
"The repository you want to download. This can take any of the following forms:

GitHub:
  user/repo
  github:user/repo
  https://github.com/user/repo

GitLab:
    gitlab:user/repo
    https://gitlab.com/user/repo

BitBucket:
    bitbucket:user/repo
    https://bitbucket.org/user/repo

You can clone a specific subdirectory instead of the entire repository:
    user/repo/subdirectory

And you can specify a branch (defaults to HEAD), tag, or commit from any of the above forms using:
    user/repo#branch
    user/repo#v1.0.0
    user/repo#abcd1234

")

                .required(true)
                .index(1)
                .validator(degit_rs::validate_src),
        )
        .arg(
            Arg::with_name("dest")
                .help("download location")
                .long_help("The destination directory. This is where the contents of the repository will be downloaded.")
                .required(false)
                .index(2)
                .validator(degit_rs::validate_dest)
                .default_value("."),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    degit_rs::degit(src, dest);
}
