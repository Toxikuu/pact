
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    about = "Presidential action tracker",
)]
pub struct Args {
    /// Number of pages to list (each page has 10)
    #[arg(short = 'p', long, value_name = "NUM")]
    pub pages: Option<u32>
}

impl Args {
    pub fn init() -> Self {
        Self::parse()
    }
}
