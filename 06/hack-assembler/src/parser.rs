use std::error::Error;
use std::fs;

// Reads all instructions to a Vec<String>, removing all whitespaces and comments
pub fn read_instructions(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let lines = fs::read_to_string(file_path)?
        .lines()
        .map(String::from)
        .map(|s| s.trim().chars().filter(|c| !c.is_whitespace()).collect()) // Remove whitespaces
        .map(|s: String| remove_comments(&s)) // Remove comments
        .filter(|s| s.len() > 0) // Filter out empty lines
        .collect();

    Ok(lines)
}

// Returns a new string with the comment removed
fn remove_comments(s: &str) -> String {
    let offset = s.find("//").unwrap_or(s.len());
    let mut new = s.to_string();
    new.truncate(offset);
    new
}

pub fn parse_instructions<'a, 'b>(
    instructions: &'a[String], 
    symbols: &'b mut crate::symbols::Symbols
) -> Vec<Vec<&'a str>> {
    let mut parsed_instructions: Vec<Vec<&str>> = vec![];
    let mut pc = 0;

    for instr in instructions.iter() {
        if instr.starts_with("@") {
            // A-instruction
            let v: Vec<&str> = instr.split("@").collect();
            let new = vec!["@", v[1]];
            parsed_instructions.push(new);
            pc += 1;
        } else if instr.starts_with("(") {
            // Label
            let v: Vec<&str> = instr.split(|c| c == '(' || c == ')').collect();
            symbols.insert_label(v[1], pc);
        } else {
            // C-instruction
            let v: Vec<&str> = instr.split(|c| c == '=' || c == ';').collect();
            parsed_instructions.push(v);
            pc += 1;
        }
    }

    parsed_instructions
}
