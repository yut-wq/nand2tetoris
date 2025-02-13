use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable(HashMap<String, u16>);

impl SymbolTable {
    pub fn new() -> Self {
        // 定義済みシンボルを格納する
        let table = HashMap::from([
            ("R0".to_string(), 0),
            ("R1".to_string(), 1),
            ("R2".to_string(), 2),
            ("R3".to_string(), 3),
            ("R4".to_string(), 4),
            ("R5".to_string(), 5),
            ("R6".to_string(), 6),
            ("R7".to_string(), 7),
            ("R8".to_string(), 8),
            ("R9".to_string(), 9),
            ("R10".to_string(), 10),
            ("R11".to_string(), 11),
            ("R12".to_string(), 12),
            ("R13".to_string(), 13),
            ("R14".to_string(), 14),
            ("R15".to_string(), 15),
            ("SP".to_string(), 0),
            ("LCL".to_string(), 1),
            ("ARG".to_string(), 2),
            ("THIS".to_string(), 3),
            ("THAT".to_string(), 4),
            ("SCREEN".to_string(), 16384),
            ("KBD".to_string(), 24576),
        ]);
        SymbolTable(table)
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        let SymbolTable(table) = self;
        table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        let SymbolTable(table) = self;
        table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> u16 {
        let SymbolTable(table) = self;
        *table.get(symbol).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn symbol_table_add_entry_success() {
        let mut table = SymbolTable::new();
        table.add_entry("test".to_string(), 8);

        let table = table.0;

        assert!(table.contains_key("test"));
        assert_eq!(*table.get("test").unwrap(), 8);
    }

    #[test]
    fn symbol_table_contains_success() {
        let table = HashMap::from([
            ("Mercury".to_string(), 4),
            ("Venus".to_string(), 7),
            ("Earth".to_string(), 1),
            ("Mars".to_string(), 15),
        ]);
        let table = SymbolTable(table);

        assert!(table.contains("Venus"));
    }

    #[test]
    fn symbol_table_get_address() {
        let table = HashMap::from([
            ("Mercury".to_string(), 4),
            ("Venus".to_string(), 7),
            ("Earth".to_string(), 1),
            ("Mars".to_string(), 15),
        ]);
        let table = SymbolTable(table);

        assert_eq!(table.get_address("Earth"), 1);
    }
}
