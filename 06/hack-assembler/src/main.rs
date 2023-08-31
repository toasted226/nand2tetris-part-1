use std::{env, process};

use hack_assembler::Assembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let assembler: Assembler = Assembler::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = assembler.assemble() {
        eprintln!("Problem while assembling: {e}");
    }
}
