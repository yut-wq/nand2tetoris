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

    fn jump(&self, jump: &str) -> String {
        if jump.is_empty() {
            return "000".to_string();
        }

        match jump {
            "JGT" => "001".to_string(),
            "JEQ" => "010".to_string(),
            "JGE" => "011".to_string(),
            "JLT" => "100".to_string(),
            "JNE" => "101".to_string(),
            "JLE" => "110".to_string(),
            "JMP" => "111".to_string(),
            _ => panic!("invalid jump: {:}", jump),
        }
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

    #[test]
    fn jump_null_return_zeros() {
        let code = Code;

        let result = code.jump("");

        assert_eq!(result, "000");
    }

    #[test]
    fn jump_jgt_return_zero_zero_one() {
        let code = Code;

        let result = code.jump("JGT");

        assert_eq!(result, "001");
    }
}
