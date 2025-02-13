use std::collections::HashMap;

pub struct SymbolTable(HashMap<String, u16>);

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable(HashMap::new())
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
