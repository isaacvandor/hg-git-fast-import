use std::path::PathBuf;
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt)]
pub enum Cli {
    /// Exports single Mercurial repository to Git fast-import compatible format
    #[structopt(name = "single")]
    Single {
        /// The Mercurial repo for import to git
        #[structopt(parse(from_os_str))]
        hg_repo: PathBuf,
        /// The Git repo to import to. Creates repo if it does not exist. Otherwise saved state must exist.
        #[structopt(parse(from_os_str))]
        git_repo: Option<PathBuf>,
        /// Repository configuration in toml format.
        #[structopt(parse(from_os_str), long, short)]
        config: Option<PathBuf>,
        /// Authors remapping in toml format.
        #[structopt(parse(from_os_str), long, short)]
        authors: Option<PathBuf>,
        /// Do not clean closed Mercurial branches.
        #[structopt(name = "no-clean-closed-branches", long)]
        no_clean_closed_branches: bool,
        /// Compares resulting Git repo with Mercurial.
        #[structopt(long)]
        verify: bool,
        /// Limit high revision to import.
        #[structopt(name = "limit-high", long)]
        limit_high: Option<usize>,
    },
    /// Exports multiple Mercurial repositories to single Git repo in fast-import compatible format
    #[structopt(name = "multi")]
    Multi {
        /// The Git repo to import to. Creates repo if it does not exist. Otherwise saved state must exist.
        #[structopt(parse(from_os_str))]
        git_repo: Option<PathBuf>,
        /// Repositories configuration in toml format.
        #[structopt(parse(from_os_str), long, short)]
        config: PathBuf,
        /// Authors remapping in toml format.
        #[structopt(parse(from_os_str), long, short)]
        authors: Option<PathBuf>,
        /// Do not clean closed Mercurial branches.
        #[structopt(name = "no-clean-closed-branches", long)]
        no_clean_closed_branches: bool,
        /// Compares resulting Git repo with Mercurial (only final state with subfolders).
        #[structopt(long)]
        verify: bool,
    },
    /// Generates completion scripts for your shell
    #[structopt(
        name = "completions",
        raw(setting = "structopt::clap::AppSettings::Hidden")
    )]
    Completions {
        /// The shell to generate the script for
        #[structopt(raw(possible_values = r#"&["bash", "fish", "zsh"]"#))]
        shell: structopt::clap::Shell,
    },
}
