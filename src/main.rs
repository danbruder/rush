mod cmd;

use self::cmd::{Cmd, ParseError};
use std::io::{self, Write};
use std::process::Command;

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
