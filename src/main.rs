use std::{fs::read_to_string, io::Write, path::PathBuf};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input File
    #[clap()]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let mut program = match read_to_string(&args.input_file) {
        Ok(s) => s,
        Err(e) => {
            println!("error reading {}: {}", &args.input_file, e);
            return;
        }
    };

    // so programs dont have to end with a newline
    program.push('\n');

    let assembler = hack_asm::Assembler::new(&program);
    let result = match assembler.assemble() {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut outfile_path = PathBuf::from(args.input_file);
    outfile_path.set_extension("hack");

    let mut outfile = match std::fs::File::create(outfile_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    for line in result {
        match writeln!(outfile, "{:016b}", line) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }
}
