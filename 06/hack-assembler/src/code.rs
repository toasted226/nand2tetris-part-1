use crate::symbols::Symbols;

pub fn translate_instruction(instruction: &[&str], symbols: &mut Symbols) -> String {
    let mut binary = String::new();

    if instruction[0] == "@" {
        // A-instruction
        let dec: u16;
        dec = match instruction[1].parse::<u16>() {
            Ok(d) => d,
            Err(_) => symbols.lookup(instruction[1])
        };
        binary = String::from(format!("{dec:016b}"));
    } else {
        // C-instruction
        binary.push_str("111");
        let comp: String;
        let a: bool;
        let dest: String;
        let jump: String;
        
        if instruction.len() == 2 {
            if instruction[1].to_lowercase().contains("j") {
                (comp, a) = translate_comp(instruction[0]); // get comp and a bit
                dest = String::from("000"); // not stored
                jump = translate_jump(instruction[1]); // get jump bits
            } else {
                dest = translate_dest(instruction[0]); // not stored
                (comp, a) = translate_comp(instruction[1]); // get comp and a bit
                jump = String::from("000"); // no jump
            }
        } else {
            dest = translate_dest(instruction[0]); // get dest bits
            (comp, a) = translate_comp(instruction[1]); // get a and comp bits
            jump = translate_jump(instruction[2]); // get jump bits
        }

        let a = if a == true { "1" } else { "0" };
        binary.push_str(a); // 0 or 1
        binary.push_str(&comp); // c1 c2 c3 c4 c5 c6
        binary.push_str(&dest); // d1 d2 d3
        binary.push_str(&jump); // j1 j2 j3
    }

    binary
}

fn translate_dest(s: &str) -> String {
    match s {
        "M" => String::from("001"),
        "D" => String::from("010"),
        "MD" => String::from("011"),
        "A" => String::from("100"),
        "AM" => String::from("101"),
        "AD" => String::from("110"),
        "AMD" => String::from("111"),
        _ => String::new()
    }
}

fn translate_comp(s: &str) -> (String, bool) {
    match s {
        "0" => (String::from("101010"), false),
        "1" => (String::from("111111"), false),
        "-1" => (String::from("111010"), false),
        "D" => (String::from("001100"), false),
        "A" => (String::from("110000"), false),
        "!D" => (String::from("001101"), false),
        "!A" => (String::from("110001"), false),
        "-D" => (String::from("001111"), false),
        "-A" => (String::from("110011"), false),
        "D+1" => (String::from("011111"), false),
        "A+1" => (String::from("110111"), false),
        "D-1" => (String::from("001110"), false),
        "A-1" => (String::from("110010"), false),
        "D+A" => (String::from("000010"), false),
        "A+D" => (String::from("000010"), false),
        "D-A" => (String::from("010011"), false),
        "A-D" => (String::from("000111"), false),
        "D&A" => (String::from("000000"), false),
        "A&D" => (String::from("000000"), false),
        "D|A" => (String::from("010101"), false),
        "A|D" => (String::from("010101"), false),
        "M" => (String::from("110000"), true),
        "!M" => (String::from("110001"), true),
        "-M" => (String::from("110011"), true),
        "M+1" => (String::from("110111"), true),
        "M-1" => (String::from("110010"), true),
        "D+M" => (String::from("000010"), true),
        "M+D" => (String::from("000010"), true),
        "D-M" => (String::from("010011"), true),
        "M-D" => (String::from("000111"), true),
        "D&M" => (String::from("000000"), true),
        "M&D" => (String::from("000000"), true),
        "D|M" => (String::from("010101"), true),
        "M|D" => (String::from("010101"), true),
        _ => (String::new(), false)
    }
}

fn translate_jump(s: &str) -> String {
    match s {
        "JGT" => String::from("001"),
        "JEQ" => String::from("010"),
        "JGE" => String::from("011"),
        "JLT" => String::from("100"),
        "JNE" => String::from("101"),
        "JLE" => String::from("110"),
        "JMP" => String::from("111"),
        _ => String::new(),
    }
}
