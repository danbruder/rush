#![feature(box_patterns)]
#![feature(try_from)]

mod cmd;

use self::cmd::{Expression, ParseError};
use std::convert::TryFrom;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;

        match Expression::try_from(line.as_ref()) {
            Ok(expr) => {
                expr.run();
            }

            Err(ParseError::EmptyLine) => {}
        }
    }
}
