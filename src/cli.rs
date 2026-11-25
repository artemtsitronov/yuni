use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "yuni", about = "Hide data inside algebraic expressions")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Encode { message: String },
    Decode { expr: String },
}
