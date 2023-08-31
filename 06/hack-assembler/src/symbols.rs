use std::collections::HashMap;

pub struct Symbols {
    pub table: HashMap<String, u16>,
    address: u16,
}

impl Symbols {
    pub fn new() -> Self {
        let mut table: HashMap<String, u16> = HashMap::new();

        // Add R0..R15
        for i in 0..15 {
            table.insert(String::from("R".to_owned() + i.to_string().as_str()), i);
        }

        table.insert(String::from("SCREEN"), 16384);
        table.insert(String::from("KBD"), 24576);
        table.insert(String::from("SP"), 0);
        table.insert(String::from("LCL"), 1);
        table.insert(String::from("ARG"), 2);
        table.insert(String::from("THIS"), 3);
        table.insert(String::from("THAT"), 4);

        Self { table, address: 0 }
    }

    pub fn insert_label(&mut self, k: &str, v: u16) {
        self.table.insert(k.to_string(), v);
    }

    pub fn lookup(&mut self, k: &str) -> u16 {
        if self.table.contains_key(k) {
            *self.table.get(k).unwrap()
        } else {
            let val = self.address + 16;
            self.address += 1; 
            self.table.insert(k.to_string(), val);
            val
        }
    }
}
