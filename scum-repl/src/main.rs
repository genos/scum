use rustyline::{Config, EditMode, Editor, error::ReadlineError};
use scum_lib::parse;

#[derive(Debug, thiserror::Error)]
pub enum ReplError {
    #[error("Scum error: {0:?}")]
    ScumError(#[from] scum_lib::ScumError),
    #[error("Readline error: {0:?}")]
    ReadlineError(#[from] rustyline::error::ReadlineError),
}

fn main() -> Result<(), ReplError> {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl: Editor<(), _> = Editor::with_config(config)?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        match rl.readline("λ>  ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let parsed = parse(&line)?;
                println!("Parsed: {}", parsed);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}
