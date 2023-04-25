use risinglight::db::Database;
use risinglight::db::Error;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<(), Error> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new().unwrap();
    let db = Database::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // println!("Line: {}", line);
                db.run(&line)?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
