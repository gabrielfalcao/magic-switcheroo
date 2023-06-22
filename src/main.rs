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
            let (raw, _digest) = read_file(filename)?;

            let meta = MetaMagic::new(raw, "FOOBARBAZ137")?;
            println!("{}: {}", filename, hex::encode(&meta.magic()));
        }
        Commands::Jsonify { magic, filename } => {
            let (read, _) = read_file(filename)?;

            let meta = MetaMagic::new(read, magic)?;
            let json = serde_json::to_string_pretty(&meta)?;

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

// #[cfg(test)]
// mod e2e_tests {
//     use k9::assert_equal;
//     use magic_switcheroo::fs::enchant_file;
//     use magic_switcheroo::fs::read_file;
//     use magic_switcheroo::fs::restore_file;
//     use std::error::Error;
//     use std::path::Path;

//     #[test]
//     fn test_enchant_and_restore_file() -> Result<(), Box<dyn Error>> {
//         let filename: String = "o-really.png".to_string();
//         let magic: String = "THISISMAGICO".to_string();

//         let (original_contents, original_checksum) = read_file(&filename);

//         // Given an image file exists
//         assert!(Path::new(&filename).exists());

//         // When I enchant it
//         enchant_file(filename.clone(), magic.clone())?;
//         // And subsequently restore it
//         restore_file(filename.clone(), magic.clone())?;

//         // Then its checksum should match that of the original
//         let (current_contents, current_checksum) = read_file(&filename);

//         // Then its contents should also match that of the original
//         assert_equal!(current_checksum, original_checksum);
//         assert_equal!(current_contents, original_contents);
//         Ok(())
//     }
// }
