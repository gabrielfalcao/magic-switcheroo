// use std::path::{Path};
use ansi_term;
use clap::{Parser, Subcommand};
use hex;
use magic_switcheroo::fs::{enchant_file, read_file, restore_file, write_file};
use magic_switcheroo::ram::{getmark, Digest, MetaMagic};
use serde_json;
use std::error::Error;
use std::fmt;
pub use magic_switcheroo::errors::MSError;
// use magic_switcheroo::{hexdecs, CAR_SIZE};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone)]
pub struct DigestMismatch {
    expected: Digest,
    actual: Digest,
}
impl DigestMismatch {
    pub fn new(expected: Digest, actual: Digest) -> DigestMismatch {
        DigestMismatch {
            expected: expected.clone(),
            actual: actual.clone(),
        }
    }
}

impl fmt::Display for DigestMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "digest mismatch {:#?} != {:#?}",
            self.expected, self.actual
        )
    }
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help(true))]
    Enchant {
        #[arg(short, long)]
        magic: String,

        filename: String,
    },
    #[command(arg_required_else_help(true))]
    Repel {
        #[arg(short, long)]
        magic: String,

        filename: String,
    },
}

pub fn ac(code: u8) -> ansi_term::Style {
    return ansi_term::Colour::Fixed(code.try_into().unwrap()).bold();
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let _bom = getmark();

    match &args.command {
        Commands::Enchant { magic, filename } => {
            enchant_file(filename.to_string(), magic.to_string())?;
        }
        Commands::Repel { filename, magic } => {
            restore_file(filename.to_string(), magic.to_string())?;
        }
    }

    Ok(())
}
