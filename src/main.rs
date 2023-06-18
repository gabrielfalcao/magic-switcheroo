// use std::path::{Path};
use ansi_term;
use hex;
use clap::{Parser, Subcommand};
use std::fmt;
use std::error::Error;
use magic_switcheroo::ram::{getmark, MetaMagic, Digest};
use magic_switcheroo::fs::{enchant_file, restore_file, read_file, write_file};
use serde_json;
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
    Decode { filename: String },
    #[command(arg_required_else_help(true))]
    Switch {
        #[arg(short, long)]
        magic: String,

        filename: String,
    },
    #[command(arg_required_else_help(true))]
    Jsonify {
        #[arg(short, long)]
        magic: String,

        filename: String,
    },
    #[command(arg_required_else_help(true))]
    Brush {
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
        Commands::Decode { filename } => {
            let (raw, _digest) = read_file(filename);

            let meta = MetaMagic::new(raw, "FOOBARBAZ137")?;
            println!("{}: {}", filename, hex::encode(&meta.magic()));
        }
        Commands::Jsonify { magic, filename } => {
            let (read, _) = read_file(filename);

            let meta = MetaMagic::new(read, magic)?;
            let json = serde_json::to_string_pretty(&meta.humanized())?;

            write_file(filename.clone(), json.as_bytes().to_vec())?;
        }
        Commands::Switch { magic, filename } => {
            enchant_file(filename.to_string(), magic.to_string())?;
        }
        Commands::Brush { filename, magic } => {
            restore_file(filename.to_string(), magic.to_string())?;
        }
    }

    Ok(())
}
