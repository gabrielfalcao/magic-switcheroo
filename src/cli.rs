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

    #[command(arg_required_else_help(true), about = "dels first N bytes of file")]
    Ds(DsOps),

    #[command(arg_required_else_help(true), about = "dels last N bytes of file")]
    De(DeOps),

    #[command(arg_required_else_help(true), about = "reads first N bytes of file")]
    Rs(RsOps),

    #[command(arg_required_else_help(true), about = "reads last N bytes of file")]
    Re(ReOps),

    #[command(
        arg_required_else_help(true),
        about = "reads every N bytes chunking with linebreak"
    )]
    Ch(ChOps),

    #[command(arg_required_else_help(true), about = "Rev")]
    Rev(RevOps),
}

#[derive(Args, Debug)]
pub struct EOps {
    #[arg(short, long)]
    pub magic: String,
    #[arg()]
    pub filename: String,
}

#[derive(Args, Debug)]
pub struct ROps {
    #[arg(short, long)]
    pub magic: String,
    #[arg()]
    pub filename: String,
}

#[derive(Args, Debug)]
pub struct GpOps {
    #[arg()]
    pub filename: String,
    #[arg()]
    pub bytes: Vec<String>,
}

#[derive(Args, Debug)]
pub struct GsOps {
    #[arg()]
    pub filename: String,
    #[arg()]
    pub bytes: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DsOps {
    #[arg()]
    pub filename: String,
    #[arg(default_value_t = 0)]
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct DeOps {
    #[arg()]
    pub filename: String,
    #[arg(default_value_t = 0)]
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct RsOps {
    #[arg()]
    pub filename: String,
    #[arg(default_value_t = 0)]
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct ReOps {
    #[arg()]
    pub filename: String,
    #[arg(default_value_t = 0)]
    pub amount: usize,
}

#[derive(Args, Debug)]
pub struct ChOps {
    #[arg()]
    pub filename: String,
    #[arg(default_value_t = 0)]
    pub amount: usize,

    #[arg(short, long)]
    pub skip_chunks: Option<usize>,
}

#[derive(Args, Debug)]
pub struct RevOps {
    #[arg()]
    pub filename: String,
}
