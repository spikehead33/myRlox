mod compiler;
mod compiler_error;
mod location;
mod utils;

use compiler_error::CompilerError;
use std::{env, path::Path};
use utils::CompilerResult;

fn run(filepath: Option<&Path>, code: String) -> CompilerResult<()> {
    let filepath = if let Some(fp) = filepath {
        fp
    } else {
        Path::new("")
    };

    // let lexer = compiler::Lexer::new(filepath, code);
    Ok(())
}

fn run_file(filepath: &Path) -> CompilerResult<()> {
    let code = std::fs::read_to_string(filepath).map_err(CompilerError::NoSuchFileError)?;
    run(Some(filepath), code)
}

fn run_prompt() -> CompilerResult<()> {
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

        // if let Err(e) = run(expression.clone()) {
        //     eprint!("{:?}", e);
        // }

        expression.clear();
    }

    Ok(())
}

fn main() -> CompilerResult<()> {
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
