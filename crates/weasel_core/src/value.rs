#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::super::{WastParser, *};

    #[test]
    fn test_num_parse() {
        let mut output = WastParser::parse(Rule::num, "123").unwrap();
        assert_eq!(output.next().unwrap().as_str(), "123");

        let mut output = WastParser::parse(Rule::num, "123_45").unwrap();
        assert_eq!(output.next().unwrap().as_str(), "123_45");

        let mut output = WastParser::parse(Rule::nat, "0xE123").unwrap();
        assert_eq!(output.next().unwrap().as_str(), "0xE123");

        let mut output = WastParser::parse(Rule::nat, "0xE123_45").unwrap();
        assert_eq!(output.next().unwrap().as_str(), "0xE123_45");
    }
}
