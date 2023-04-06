use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {

    /// View target markdown file
    markdown_file: String,
}

impl Cli {
    pub(crate) fn parse_fn() -> Cli {
        Cli::parse()
    }

    pub(crate) fn get_arg(&self) -> Option<&String> {
        Some(&self.markdown_file)
    }
}
