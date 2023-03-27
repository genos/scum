use rustyline::{error::ReadlineError, Config, EditMode, Editor};
use scum_lib::read;

fn main() -> Result<(), ReadlineError> {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl: Editor<(), _> = Editor::with_config(config)?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        match rl.readline("λ>  ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match read(&line) {
                    Ok(parsed) => {
                        println!("Parsed:");
                        for expression in parsed {
                            println!("{expression}");
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                continue;
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}
