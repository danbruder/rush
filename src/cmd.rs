use std::convert::TryFrom;

// A command consists of a binary and its arguments
#[derive(Debug, PartialEq)]
pub struct Cmd<'a> {
    pub binary: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for Cmd<'a> {
    type Error = ParseError;

    // Extract the command and its arguments from the commandline
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let mut parts = line.split_whitespace();
        let binary = parts.nth(0).ok_or_else(|| ParseError::EmptyLine)?;
        let args = parts.collect();

        Ok(Cmd { binary, args })
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
        assert_eq!(
            Cmd::try_from("echo").unwrap(),
            Cmd {
                binary: "echo",
                args: vec![]
            }
        );
    }

    #[test]
    fn test_binary_with_arguments() {
        assert_eq!(
            Cmd::try_from("echo 1 2 3").unwrap(),
            Cmd {
                binary: "echo",
                args: vec!["1", "2", "3"]
            }
        );
    }
}
