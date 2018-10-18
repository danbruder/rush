use std::convert::TryFrom;
use std::ffi::OsStr;
use std::str::SplitWhitespace;

// A command consists of a binary and its arguments
#[derive(Debug)]
pub struct Cmd<'a> {
    pub binary: &'a OsStr,
    pub args: LineIter<'a>,
}

impl<'a> TryFrom<&'a str> for Cmd<'a> {
    type Error = ParseError;

    // Extract the command and its arguments from the commandline
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let mut line_iter = LineIter(line.split_whitespace());

        let binary = line_iter.next()
            .map(OsStr::new)
            .ok_or(ParseError::EmptyLine)?;

        Ok(Cmd {
            binary,
            args: line_iter,
        })
    }
}

#[derive(Debug)]
pub struct LineIter<'a>(SplitWhitespace<'a>);

impl<'a> std::iter::Iterator for LineIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyLine,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        assert_eq!(Cmd::try_from("").unwrap_err(), ParseError::EmptyLine);
    }

    #[test]
    fn test_single_binary() {
        let mut cmd = Cmd::try_from("echo").unwrap();

        assert_eq!(cmd.binary, OsStr::new("echo"));
        assert_eq!(cmd.args.next(), None);
    }

    #[test]
    fn test_binary_with_arguments() {
        let mut cmd = Cmd::try_from("echo 1 2 3").unwrap();

        assert_eq!(cmd.binary, OsStr::new("echo"));
        assert_eq!(cmd.args.next(), Some("1"));
        assert_eq!(cmd.args.next(), Some("2"));
        assert_eq!(cmd.args.next(), Some("3"));
        assert_eq!(cmd.args.next(), None);
    }
}
