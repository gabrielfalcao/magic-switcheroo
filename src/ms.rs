// use std::path::{Path};
use ansi_term;
//use clap::{Parser, Subcommand};
pub use magic_switcheroo::errors::MSError;
use magic_switcheroo::cli::{Engine, Commands};
use magic_switcheroo::fs::{
    delete_end_file,
    delete_start_file,
    read_end_file,
    read_start_file,
    read_file_chunks,
    enchant_file,
    restore_file,
    prefix_file,
    suffix_file,
    read_file,
    write_file,
};
use magic_switcheroo::ram::{Digest};
use std::error::Error;
use std::fmt;
// use magic_switcheroo::{hexdecs, CAR_SIZE};


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

pub fn ac(code: u8) -> ansi_term::Style {
    return ansi_term::Colour::Fixed(code.try_into().unwrap()).bold();
}

pub fn main() -> Result<(), Box<dyn Error>> {
    match &Engine::start() {
        Commands::E(ops) => {
            enchant_file(ops.filename.to_string(), ops.magic.to_string())?;
        }
        Commands::R(ops) => {
            restore_file(ops.filename.to_string(), ops.magic.to_string())?;
        }
        Commands::Gp(ops) => {
            prefix_file(ops.filename.to_string(), ops.bytes.to_vec())?;
            eprintln!("gp {}", ops.filename);
        }
        Commands::Gs(ops) => {
            suffix_file(ops.filename.to_string(), ops.bytes.to_vec())?;
            eprintln!("gs {}", ops.filename);
        }
        Commands::Ds(ops) => {
            let start = delete_start_file(ops.filename.to_string(), ops.amount)?;
            println!("{}", start.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::De(ops) => {
            let end = delete_end_file(ops.filename.to_string(), ops.amount)?;
            println!("{}", end.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::Rs(ops) => {
            let start = read_start_file(ops.filename.to_string(), ops.amount)?;
            println!("{}", start.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::Re(ops) => {
            let end = read_end_file(ops.filename.to_string(), ops.amount)?;
            println!("{}", end.iter().map(|x| format!("0x{:02x}", x)).collect::<Vec<String>>().join(" "));
        }
        Commands::Ch(ops) => {
            let chunks = read_file_chunks(ops.filename.to_string(), ops.amount, ops.skip_chunks)?;
            for chunk in chunks {
                println!("{}", chunk.iter().map(|x| format!("{:02x}", x)).collect::<Vec<String>>().join(""));
            }
        }
        Commands::Rev(ops) => {
            let (mut bytes, crc32) = read_file(&ops.filename.to_string())?;
            bytes.reverse();
            let output = format!("{}.{}", &ops.filename, hex::encode(crc32));
            write_file(output, bytes)?;
        }
    }
    Ok(())
}
