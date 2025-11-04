use clap::{Arg, ArgAction, Command, crate_version};

fn main() {
    let matches = Command::new("degit-rs")
        .version(crate_version!())
        .about("Download the contents of a git repository without cloning it.")
        .arg(
            Arg::new("src")
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
                .value_parser(degit_rs::validate_src),
        )
        .arg(
            Arg::new("dest")
                .help("download location")
                .long_help("The destination directory. This is where the contents of the repository will be downloaded.")
                .required(false)
                .index(2)
                .value_parser(degit_rs::validate_dest)
                .default_value("."),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .help("Sets the level of verbosity")
                .action(ArgAction::Count),
        )
        .get_matches();

    let src = matches.get_one::<String>("src").unwrap();
    let dest = matches.get_one::<String>("dest").unwrap();

    degit_rs::degit(src, dest);
}
