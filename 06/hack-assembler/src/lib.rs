use std::{path::Path, error::Error, fs::File, io::Write};

mod parser;
mod code;
mod symbols;

pub struct Assembler {
    file_path: String,
    output_path: String,
}

impl Assembler {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = args[1].clone();
        let pathbuf = Path::new(&file_path).with_extension("");
        let filename = pathbuf.to_str().unwrap();
        let output_path = String::from(filename.to_owned() + ".hack");

        Ok(Self { file_path, output_path })
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn output_path(&self) -> &str {
        &self.output_path
    }

    pub fn assemble(&self) -> Result<(), Box<dyn Error>> {
        let instructions = parser::read_instructions(&self.file_path)?;
        let mut symbols = symbols::Symbols::new();
        let parsed = parser::parse_instructions(&instructions, &mut symbols);

        let mut file = File::create(&self.output_path)?; 

        for i in parsed.iter() {
            let code = String::from(code::translate_instruction(i, &mut symbols) + "\n");
            file.write(code.as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_correct_paths() {
        let args = vec![String::from("app.exe"), String::from("pong.asm")];
        let assembler = Assembler::new(&args).expect("if this fails, something's wrong");

        assert_eq!("pong.asm", assembler.file_path);
        assert_eq!("pong.hack", assembler.output_path);
    }
}
