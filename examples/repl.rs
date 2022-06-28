use std::{env, error::Error, fs};

use rustyline::{error::ReadlineError, Editor};
use wmd::lexer::Lexer;

const HISTORY: &'static str = ".wmd-history.txt";

fn repl() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY).unwrap_or(());

    loop {
        let readline = rl.readline("wmd> ");
        match readline {
            Ok(line) => {
                let lexer = Lexer::new(&line);
                let tokens = lexer.scan_tokens();
                println!("{tokens:#?}");
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {err:?}");
                break;
            }
        }
    }

    rl.save_history(HISTORY)?;
    Ok(())
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let src = fs::read_to_string(path)?;
    println!("{src}");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        run_file(&args[1])
    } else {
        repl()
    }
}
