mod compiler_error;

use compiler_error::CompilerError;
use std::{env, path::Path};

fn run(code: String) -> Result<(), CompilerError> {
    Ok(())
}

fn run_file(filepath: &Path) -> Result<(), CompilerError> {
    let code = std::fs::read_to_string(filepath)
                .map_err(CompilerError::NoSuchFileError)?;
    run(code)
}

fn run_prompt() -> Result<(), CompilerError> {
    let mut expression = String::new();
    let stdin = std::io::stdin();

    loop {
        print!("rlox> ");

        if let Err(e) = stdin.read_line(&mut expression) {
            eprint!("{:?}", e);
        }

        if expression == "exit!".to_string() {
            break;
        }

        if let Err(e) = run(expression.clone()) {
            eprint!("{:?}", e);
        }

        expression.clear();
    }

    Ok(())
}

fn main() -> Result<(), CompilerError> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Usage: rlox [script]");
    }

    if args.len() == 2 {
        let filepath = Path::new(args[0].as_str());
        run_file(filepath)
    } else {
        run_prompt()
    }
}
