#![deny(unsafe_code)]
use rustyline::{error::ReadlineError, Config, EditMode, Editor};
use scum::Scum;

fn main() -> Result<(), ReadlineError> {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut scum = Scum::default();
    let mut rl: Editor<(), _> = Editor::with_config(config)?;
    println!(
        r"
  ___ ______ ____ _ 
 (_-</ __/ // /  ' \
/___/\__/\_,_/_/_/_/
"
    );
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        match rl.readline("λ>  ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match scum.read_eval_print(&line) {
                    Ok(expression) => {
                        println!("{expression}");
                    }
                    Err(e) => {
                        eprintln!("{e}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("^C");
            }
            Err(ReadlineError::Eof) => {
                eprintln!("Bye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}
