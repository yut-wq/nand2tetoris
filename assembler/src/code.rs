#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Code;

impl Code {
    fn dest(&self, dest: &str) -> String {
        if dest.is_empty() {
            return "000".to_string();
        }

        let mut bin_code = String::new();

        if dest.contains("A") {
            bin_code.push('1')
        } else {
            bin_code.push('0');
        }

        if dest.contains("D") {
            bin_code.push('1')
        } else {
            bin_code.push('0');
        }

        if dest.contains("M") {
            bin_code.push('1')
        } else {
            bin_code.push('0');
        }

        bin_code
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

    #[test]
    fn dest_ad_return_one_one_zero() {
        let code = Code;

        let result = code.dest("AD");

        assert_eq!(result, "110");
    }
}
