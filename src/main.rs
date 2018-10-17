use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut line = String::new();
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        let command: String = line.split_whitespace().take(1).collect();
        let args: Vec<_> = line.split_whitespace().skip(1).collect();

        match Command::new(command).args(&args).output() {
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
        }
    }
}
