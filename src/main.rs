use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, PartialEq)]
enum ParseError {
    EmptyLine,
}

// A command consists of a binary and its arguments
#[derive(Debug, PartialEq)]
struct Cmd<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    // Extract the command and its arguments from the commandline
    fn parse_from(line: &'a str) -> Result<Self, ParseError> {
        let mut parts = line.split_whitespace();
        let binary = parts.nth(0).ok_or_else(|| ParseError::EmptyLine)?;
        let args = parts.collect();

        Ok(Cmd { binary, args })
    }
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;

        match Cmd::parse_from(&line) {
            Ok(cmd) => match Command::new(cmd.binary).args(cmd.args).output() {
                Ok(output) => {
                    if output.status.success() {
                        io::stdout().write(&output.stdout)?;
                    } else {
                        io::stderr().write(&output.stderr)?;
                    }
                }

                Err(e) => {
                    eprintln!("{}", e);
                }
            },

            Err(ParseError::EmptyLine) => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        assert_eq!(Cmd::parse_from("").unwrap_err(), ParseError::EmptyLine);
    }

    #[test]
    fn test_single_binary() {
        assert_eq!(
            Cmd::parse_from("echo").unwrap(),
            Cmd {
                binary: "echo",
                args: vec![]
            }
        );
    }

    #[test]
    fn test_binary_with_arguments() {
        assert_eq!(
            Cmd::parse_from("echo 1 2 3").unwrap(),
            Cmd {
                binary: "echo",
                args: vec!["1", "2", "3"]
            }
        );
    }
}
