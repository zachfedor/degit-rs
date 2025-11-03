# degit-rs

Forked from [psnszsn/degit-rs](https://github.com/psnszsn/degit-rs) with additional features to support:
- Tags
- Branches
- Commits

degit-rs is a rust rewrite of [degit](https://github.com/Rich-Harris/degit/). It downloads copies of git repositories from the internet, without the need for `git clone`. This is faster, since it does not download the `.git` folder (which contains all the git history) and allows you to initialize a new repository afterwards. It is useful for downloading project boilerplate templates.

[![Crates.io](https://img.shields.io/crates/v/degit.svg)](https://crates.io/crates/degit)

## Installation
```
cargo install degit
```

## Usage examples

Simplest usage downloads the main branch on GitHub to the current working directory, or to a target directory if you provide it.

```
degit user/repo
degit user/repo path/to/project
```

You can also download from GitLab and Bitbucket using `gitlab:user/repo` and `bitbucket:user/repo`, respectively. Or you can specify something other than the main branch with:

```
degit user/repo#dev         # by branch
degit user/repo#v1.2.3      # by tag
degit user/repo#1234abcd    # by commit hash
```

## Advantages over original degit
* does not require nodejs
* does not create files in your home directory
* supports hosted gitlab instances
* progress bar
