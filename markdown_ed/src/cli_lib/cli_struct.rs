use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {

    /// View target markdown file
    #[arg(short, long, value_name = "FILE NAME")]
    markdown_file: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub(crate) fn parse_fn() -> Cli {
        Cli::parse()
    }

    pub(crate) fn get_arg(&self) -> Option<&String> {
        Some(&self.markdown_file)
    }
}


#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}