use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(short, long)]
    pub file_path: String,
}
