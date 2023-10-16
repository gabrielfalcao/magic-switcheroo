// use crate::coreio::ensure_dir_exists;
// use crate::errors::Error;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Engine {
    #[command(subcommand)]
    pub commands: Commands,
}

impl Engine {
    pub fn start() -> Commands {
        Self::parse().commands
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        arg_required_else_help(true),
        about = "enchants a file with the given 96 bits magic word"
    )]
    E(EOps),

    #[command(
        arg_required_else_help(true),
        about = "repels the previous enchantment applied to a file with the given 96 bits magic word"
    )]
    R(ROps),

    #[command(
        arg_required_else_help(true),
        about = "grafts start of file with given bytes"
    )]
    Gp(GpOps),

    #[command(
        arg_required_else_help(true),
        about = "grafts end of file with given bytes"
    )]
    Gs(GsOps),

    #[command(
        arg_required_else_help(true),
        about = "dels first N bytes of file"
    )]
    Ds(DsOps),

    #[command(
        arg_required_else_help(true),
        about = "dels last N bytes of file"
    )]
    De(DeOps),

    #[command(
        arg_required_else_help(true),
        about = "reads first N bytes of file"
    )]
    Rs(RsOps),

    #[command(
        arg_required_else_help(true),
        about = "reads last N bytes of file"
    )]
    Re(ReOps),
}

#[derive(Args, Debug)]
pub struct EOps {
    #[arg(short, long)]
    pub magic: String,
    pub filename: String,
}

#[derive(Args, Debug)]
pub struct ROps {
    #[arg(short, long)]
    pub magic: String,
    pub filename: String,
}

#[derive(Args, Debug)]
pub struct GpOps {
    pub filename: String,
    pub bytes: Vec<String>,
}

#[derive(Args, Debug)]
pub struct GsOps {
    pub filename: String,
    pub bytes: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DsOps {
    pub filename: String,
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct DeOps {
    pub filename: String,
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct RsOps {
    pub filename: String,
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct ReOps {
    pub filename: String,
    pub amount: usize,
}
