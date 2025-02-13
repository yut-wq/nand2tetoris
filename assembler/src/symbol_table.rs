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
}
