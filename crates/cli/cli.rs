use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None)]
pub struct Args {
    #[arg(long, short = 'v')]
    pub version: bool,

    #[arg(long, global = true)]
    pub debug: bool,
}
