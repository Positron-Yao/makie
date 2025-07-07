use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,

    #[arg()]
    pub arg: Option<String>,
}

#[derive(Subcommand)]
pub enum CliCommand {
    Clean,
}
