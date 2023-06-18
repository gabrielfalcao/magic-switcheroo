// use std::path::{Path};
use ansi_term;
use hex;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use magic_switcheroo::ram::{crc32, getmark, MetaMagic};
use magic_switcheroo::fs::{enchant_file, restore_file};
use serde_json;
// use magic_switcheroo::{hexdecs, CAR_SIZE};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

pub fn log_error_and_exit(message: &str) {
    eprintln!("{}", ac(0xac).paint(message));
    std::process::exit(0x54);
}
pub fn read_file_into_vec(filename: &String, contents: &mut Vec<u8>) {
    let mut f = File::open(filename).unwrap();
    f.read_to_end(contents).expect(&format!("failed to read file '{}'", filename));
}
pub fn read_file(filename: &String) -> (Vec<u8>, Vec<u8>) {
    let mut contents = Vec::new();
    read_file_into_vec(filename, &mut contents);
    return (contents.clone(), crc32(&contents));
}
pub fn main() {
    let args = Cli::parse();
    let _bom = getmark();

    match &args.command {
        Commands::Decode { filename } => {
            let (raw, _digest) = read_file(filename);

            let meta = MetaMagic::new(raw, "FOOBARBAZ137");
            println!("{}: {}", filename, hex::encode(&meta.magic()));
        }
        Commands::Jsonify { magic, filename } => {
            let (read, fdigest) = read_file(filename);

            let meta = MetaMagic::new(read, magic);
            if fdigest != meta.odigest() {
                log_error_and_exit("failed to calculate digest of given file");
                return;
            }

            let json = serde_json::to_string_pretty(&meta.humanized()).unwrap();

            let mut file = File::create(filename).expect("failed to create new file");
            file.write_all(&json.into_bytes()).expect("failed to write enchanted data into file");

        }
        Commands::Switch { magic, filename } => {
            enchant_file(filename.to_string(), magic.to_string()).expect("failed to enchant file");
        }
        Commands::Brush { filename, magic } => {
            restore_file(filename.to_string(), magic.to_string()).expect("failed to restore file");
        }
    }
}
