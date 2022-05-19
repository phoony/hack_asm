#[derive(Debug)]
pub struct ProgramLine<'a> {
    line: &'a str,
    rest: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum LineType {
    AInstruction,
    CInstruction,
    Label,
    Other,
}

impl<'a> ProgramLine<'a> {
    pub fn line_type(&self) -> LineType {
        if self.rest.starts_with("//") {
            return LineType::Other;
        }

        match self.rest.chars().next() {
            Some('@') => LineType::AInstruction,
            Some('(') => LineType::Label,
            Some(_) => LineType::CInstruction,
            None => LineType::Other,
        }
    }

    fn longest_match<'b, 'c>(&'a self, options: &[&'b str], followed_by: &'c str) -> &'b str {
        let mut longest_len = 0;
        let mut longest_str = "";

        for option in options {
            let len = option.len();
            if self.rest.starts_with(option)
                && self.rest[len..].starts_with(followed_by)
                && len > longest_len
            {
                longest_len = len;
                longest_str = option;
            }
        }

        longest_str
    }

    pub fn consume(&mut self, prefix: &str) {
        if self.rest.starts_with(prefix) {
            self.advance(prefix.len())
        }
    }

    pub fn take_longest_prefix<'b, 'c>(
        &mut self,
        prefixes: &[&'b str],
        followed_by: &'c str,
    ) -> &'b str {
        let longest = self.longest_match(prefixes, followed_by);
        if !longest.is_empty() {
            self.advance(longest.len() + followed_by.len());
        }
        longest
    }

    pub fn take_valid(&mut self, valid: &str) -> &'a str {
        let chars = self.rest.chars();
        let mut count = 0;
        for char in chars {
            if valid.contains(char) {
                count += 1;
            } else {
                break;
            }
        }
        let result = &self.rest[..count];
        self.advance(count);

        result
    }

    pub fn rest_is_comment_or_empty(&self) -> bool {
        self.rest.trim_start().is_empty() || self.rest.trim_start().starts_with("//")
    }

    pub fn peek(&self) -> Option<char> {
        self.rest.chars().next()
    }

    fn advance(&mut self, by: usize) {
        self.rest = &self.rest[by..];
    }
}

impl<'a> From<&'a str> for ProgramLine<'a> {
    fn from(val: &'a str) -> Self {
        ProgramLine {
            line: val,
            rest: val.trim_start(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_instruction() {
        let line: ProgramLine = "A=BD".into();
        assert_eq!(line.line_type(), LineType::CInstruction);
    }

    #[test]
    fn a_instruction() {
        let line: ProgramLine = "@123".into();
        assert_eq!(line.line_type(), LineType::AInstruction);
    }

    #[test]
    fn label() {
        let line: ProgramLine = "(SOME_LABEL)".into();
        assert_eq!(line.line_type(), LineType::Label);
    }

    #[test]
    fn advance() {
        let mut line: ProgramLine = "ABCDE".into();
        line.advance(3);

        assert_eq!(line.rest, "DE");
    }

    #[test]
    fn consume() {
        let mut line: ProgramLine = "ABC".into();
        line.consume("ABC");

        assert_eq!(line.rest, "")
    }

    #[test]
    fn consume_too_many() {
        let mut line: ProgramLine = "ABC".into();
        line.consume("ABCDE");

        assert_eq!(line.rest, "ABC")
    }

    #[test]
    fn longest_match() {
        let mut line: ProgramLine = "HELLO WORLD!".into();
        let prefixes = ["ABC", "HELL", "HELLO", "WORLD"];
        let result = line.take_longest_prefix(&prefixes, " ");

        assert_eq!(result, "HELLO");
        assert_eq!(line.rest, "WORLD!")
    }

    #[test]
    fn take_valid() {
        let mut line: ProgramLine = "AAABBBCDEF".into();
        let valid = "ABC";
        let result = line.take_valid(valid);

        assert_eq!(result, "AAABBBC");
        assert_eq!(line.rest, "DEF");
    }
}
