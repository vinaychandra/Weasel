use std::{path::PathBuf, process};
use structopt::StructOpt;

fn main() {
    let opt = CmdlineOptions::from_args();
    println!(
        "Reading file: {:?} with disassembly {:#?}",
        opt.input, opt.show_disassembly
    );

    if let Some(path) = opt.input {
        if !path.exists() {
            eprintln!("File does not exist: {}", path.display());
            process::exit(1);
        }
    }
}

/// Command-line interface for the weasel console.
#[derive(StructOpt, Debug)]
#[structopt(name = "weasel_console")]
pub struct CmdlineOptions {
    /// Input wat file.
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Show the disassembly of the wat file. This will
    /// print disassembly of the file line by line.
    #[structopt(long)]
    show_disassembly: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
