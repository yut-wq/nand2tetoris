#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Code;

impl Code {
    fn dest(dest: &str) -> String {
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

    fn jump(jump: &str) -> String {
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

    fn comp(comp: &str) -> String {
        match comp {
            "0" => "0101010".to_string(),
            "1" => "0111111".to_string(),
            "-1" => "0111010".to_string(),
            "D" => "0001100".to_string(),
            "A" => "0110000".to_string(),
            "!D" => "0001101".to_string(),
            "!A" => "0110001".to_string(),
            "-D" => "0001111".to_string(),
            "-A" => "0110011".to_string(),
            "D+1" => "0011111".to_string(),
            "A+1" => "0110111".to_string(),
            "D-1" => "0001110".to_string(),
            "A-1" => "0110010".to_string(),
            "D+A" => "0000010".to_string(),
            "D-A" => "0010011".to_string(),
            "A-D" => "0000111".to_string(),
            "D&A" => "0000000".to_string(),
            "D|A" => "0010101".to_string(),

            "M" => "1110000".to_string(),
            "!M" => "1110001".to_string(),
            "-M" => "1110011".to_string(),
            "M+1" => "1110111".to_string(),
            "M-1" => "1110010".to_string(),
            "D+M" => "1000010".to_string(),
            "D-M" => "1010011".to_string(),
            "M-D" => "1000111".to_string(),
            "D&M" => "1000000".to_string(),
            "D|M" => "1010101".to_string(),
            _ => panic!("invalid comp: {:}", comp),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dest_null_return_zeros() {
        let result = Code::dest("");

        assert_eq!(result, "000");
    }

    #[test]
    fn dest_m_return_zero_zero_one() {
        let result = Code::dest("M");

        assert_eq!(result, "001");
    }

    #[test]
    fn dest_ad_return_one_one_zero() {
        let result = Code::dest("AD");

        assert_eq!(result, "110");
    }

    #[test]
    fn jump_null_return_zeros() {
        let result = Code::jump("");

        assert_eq!(result, "000");
    }

    #[test]
    fn jump_jgt_return_zero_zero_one() {
        let result = Code::jump("JGT");

        assert_eq!(result, "001");
    }

    #[test]
    fn jump_jle_return_one_one_zero() {
        let result = Code::jump("JLE");

        assert_eq!(result, "110");
    }

    #[test]
    fn comp_a_plus_one_return_0110111() {
        let result = Code::comp("A+1");

        assert_eq!(result, "0110111");
    }
}
