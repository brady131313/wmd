use std::{env, error::Error, fs};

use rustyline::{error::ReadlineError, Editor};
use wmd::{interpreter::Interpreter, lexer::Lexer, parser::Parser, reporting::StdoutReporter};

const HISTORY: &'static str = ".wmd-history.txt";

fn repl() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY).unwrap_or(());

    let reporter = StdoutReporter;
    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline("wmd> ");
        match readline {
            Ok(line) => {
                let lexer = Lexer::new(&line, &reporter);
                let tokens = lexer.scan_tokens();
                // println!("{tokens:#?}");

                let mut parser = Parser::new(tokens, &reporter);
                let expr = parser.parse();
                println!("{expr:#?}");
                // match expr {
                //     Ok(expr) => match interpreter.evaluate(&expr) {
                //         Ok(res) => {
                //             println!("{res}");
                //             rl.add_history_entry(line.as_str());
                //         }
                //         Err(e) => eprintln!("{e}"),
                //     },
                //     Err(e) => eprintln!("{e:?}"),
                // }
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

    let reporter = StdoutReporter;
    let mut interpreter = Interpreter::new();

    let lexer = Lexer::new(&src, &reporter);
    let tokens = lexer.scan_tokens();

    let mut parser = Parser::new(tokens, &reporter);
    let expr = parser.parse();

    // match expr {
    //     Ok(expr) => match interpreter.evaluate(&expr) {
    //         Ok(res) => {
    //             println!("{res}");
    //         }
    //         Err(e) => eprintln!("{e}"),
    //     },
    //     Err(e) => eprintln!("{e:?}"),
    // }

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
