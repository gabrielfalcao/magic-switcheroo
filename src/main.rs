// use std::path::{Path};
use ansi_term;
use clap::{Parser, Subcommand};
pub use magic_switcheroo::errors::MSError;
use magic_switcheroo::fs::{
    delete_end_file,
    delete_start_file,
    read_end_file,
    read_start_file,
    enchant_file,
    restore_file,
    prefix_file,
    suffix_file,
};
use magic_switcheroo::ram::{getmark, Digest};
use std::error::Error;
use std::fmt;
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
    #[command(
        arg_required_else_help(true),
        about = "enchants a file with the given 96 bits magic word"
    )]
    E {
        #[arg(short, long)]
        magic: String,

        filename: String,
    },
    #[command(
        arg_required_else_help(true),
        about = "repels the previous enchantment applied to a file with the given 96 bits magic word"
    )]
    R {
        #[arg(short, long)]
        magic: String,

        filename: String,
    },
    #[command(
        arg_required_else_help(true),
        about = "grafts start of file with given bytes"
    )]
    Gp {
        filename: String,
        bytes: Vec<String>,
    },

    #[command(
        arg_required_else_help(true),
        about = "grafts end of file with given bytes"
    )]
    Gs {
        filename: String,
        bytes: Vec<String>,
    },

    #[command(
        arg_required_else_help(true),
        about = "dels first N bytes of file"
    )]
    Ds { filename: String, amount: usize },

    #[command(
        arg_required_else_help(true),
        about = "dels last N bytes of file"
    )]
    De { filename: String, amount: usize },

    #[command(
        arg_required_else_help(true),
        about = "reads first N bytes of file"
    )]
    Rs { filename: String, amount: usize },

    #[command(
        arg_required_else_help(true),
        about = "reads last N bytes of file"
    )]
    Re { filename: String, amount: usize },
}

pub fn ac(code: u8) -> ansi_term::Style {
    return ansi_term::Colour::Fixed(code.try_into().unwrap()).bold();
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let _bom = getmark();

    match &args.command {
        Commands::E { magic, filename } => {
            enchant_file(filename.to_string(), magic.to_string())?;
        }
        Commands::R { filename, magic } => {
            restore_file(filename.to_string(), magic.to_string())?;
        }
        Commands::Gp { filename, bytes } => {
            prefix_file(filename.to_string(), bytes.to_vec())?;
            eprintln!("gp {}", filename);
        }
        Commands::Gs { filename, bytes } => {
            suffix_file(filename.to_string(), bytes.to_vec())?;
            eprintln!("gs {}", filename);
        }
        Commands::Ds { filename, amount } => {
            let start = delete_start_file(filename.to_string(), *amount)?;
            println!("{}", start.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::De { filename, amount } => {
            let end = delete_end_file(filename.to_string(), *amount)?;
            println!("{}", end.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::Rs { filename, amount } => {
            let start = read_start_file(filename.to_string(), *amount)?;
            println!("{}", start.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::Re { filename, amount } => {
            let end = read_end_file(filename.to_string(), *amount)?;
            println!("{}", end.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
    }
    Ok(())
}
