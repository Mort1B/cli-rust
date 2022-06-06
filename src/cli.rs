use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    ///Write task to journal file.
    Add {
        #[structopt()]
        text: String,
    },
    /// Remove an entry from journal by position
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in journal
    List,
}

#[derive(Debug, StructOpt)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    ///Use different journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
