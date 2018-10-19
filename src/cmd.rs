use std::convert::TryFrom;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::process::Command;
use std::str::SplitWhitespace;
use std::vec::IntoIter;

pub enum Expression<'a> {
    Cmd(Cmd<'a>),

    Compound(Box<Compound<'a>>),
}

// A command consists of a binary and its arguments
#[derive(Debug)]
pub struct Cmd<'a> {
    pub binary: &'a OsStr,
    pub args: LineIter<'a>,
}

#[derive(Debug)]
pub struct LineIter<'a>(SplitWhitespace<'a>);

pub struct Compound<'a> {
    pub op: Op,
    pub left: Expression<'a>,
    pub right: Expression<'a>,
}

pub enum Op {
    Semicolon,
    And,
}

impl<'a> TryFrom<&'a str> for Expression<'a> {
    type Error = ParseError;

    // Extract the expression from the commandline
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let mut stmts = vec![];

        for stmt in line.split(';') {
            let mut cmds = vec![];

            for cmd in stmt.split("&&") {
                cmds.push(Cmd::try_from(cmd)?);
            }

            stmts.push(Self::build_and_expression(cmds.into_iter()));
        }

        Ok(Self::build_semicolon_expression(stmts.into_iter()))
    }
}

impl<'a> Expression<'a> {
    pub fn run(self) -> bool {
        match self {
            Expression::Cmd(cmd) => cmd.run(),

            Expression::Compound(compound) => match compound.op {
                Op::Semicolon => {
                    compound.left.run();
                    compound.right.run()
                }

                Op::And => compound.left.run() && compound.right.run(),
            },
        }
    }

    fn build_semicolon_expression(mut exprs: IntoIter<Expression<'a>>) -> Expression<'a> {
        assert!(exprs.len() >= 1);
        let expr_left = exprs.next().unwrap();

        if exprs.len() == 0 {
            expr_left
        } else {
            Expression::Compound(Box::new(Compound {
                op: Op::Semicolon,
                left: expr_left,
                right: Expression::build_semicolon_expression(exprs),
            }))
        }
    }

    fn build_and_expression(mut cmds: IntoIter<Cmd<'a>>) -> Expression<'a> {
        let cmd_left = cmds.next().unwrap();

        if cmds.len() == 0 {
            Expression::Cmd(cmd_left)
        } else {
            Expression::Compound(Box::new(Compound {
                op: Op::And,
                left: Expression::Cmd(cmd_left),
                right: Expression::build_and_expression(cmds),
            }))
        }
    }
}

impl<'a> TryFrom<&'a str> for Cmd<'a> {
    type Error = ParseError;

    // Extract the command and its arguments from the commandline
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let mut line_iter = LineIter(line.split_whitespace());

        let binary = line_iter
            .next()
            .map(OsStr::new)
            .ok_or(ParseError::EmptyLine)?;

        Ok(Cmd {
            binary,
            args: line_iter,
        })
    }
}

impl<'a> Cmd<'a> {
    pub fn run(self) -> bool {
        match Command::new(&self.binary).args(self.args).output() {
            Ok(output) => {
                if output.status.success() {
                    let _ = io::stdout().write(&output.stdout);
                } else {
                    let _ = io::stderr().write(&output.stderr);
                }

                output.status.success()
            }

            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
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

    #[test]
    fn test_semicolon_expression() {
        match Expression::try_from("echo 1 2 3; ls").unwrap() {
            Expression::Compound(box Compound {
                op: Op::Semicolon,
                left:
                    Expression::Cmd(Cmd {
                        binary: binary_left,
                        args: mut args_left,
                    }),

                right:
                    Expression::Cmd(Cmd {
                        binary: binary_right,
                        args: mut args_right,
                    }),
            }) => {
                assert_eq!(binary_left, OsStr::new("echo"));
                assert_eq!(args_left.next(), Some("1"));
                assert_eq!(args_left.next(), Some("2"));
                assert_eq!(args_left.next(), Some("3"));
                assert_eq!(args_left.next(), None);

                assert_eq!(binary_right, OsStr::new("ls"));
                assert_eq!(args_right.next(), None);
            }

            _ => assert!(false),
        }
    }

    #[test]
    fn test_and_expression() {
        match Expression::try_from("echo 1 2 3 && ls").unwrap() {
            Expression::Compound(box Compound {
                op: Op::And,
                left:
                    Expression::Cmd(Cmd {
                        binary: binary_left,
                        args: mut args_left,
                    }),

                right:
                    Expression::Cmd(Cmd {
                        binary: binary_right,
                        args: mut args_right,
                    }),
            }) => {
                assert_eq!(binary_left, OsStr::new("echo"));
                assert_eq!(args_left.next(), Some("1"));
                assert_eq!(args_left.next(), Some("2"));
                assert_eq!(args_left.next(), Some("3"));
                assert_eq!(args_left.next(), None);

                assert_eq!(binary_right, OsStr::new("ls"));
                assert_eq!(args_right.next(), None);
            }

            _ => assert!(false),
        }
    }
}
