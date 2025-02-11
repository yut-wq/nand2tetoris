#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Code;

impl Code {
    fn dest(&self, dest: &str) -> String {
        if dest.is_empty() {
            return "000".to_string();
        }
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dest_null_return_zeros() {
        let code = Code;

        let result = code.dest("");

        assert_eq!(result, "000");
    }

    #[test]
    fn dest_m_return_zero_zero_one() {
        let code = Code;

        let result = code.dest("M");

        assert_eq!(result, "001");
    }
}
