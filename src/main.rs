use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use ctrlc;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> rustyline::Result<()> {
    // 1. Set up Ctrl-C handler
    let running = Arc::new(AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

    // 2. Line editor + command loop
    let mut rl = DefaultEditor::new()?;
    let mut stack: Vec<i32> = Vec::new();

    while running.load(Ordering::SeqCst) {
        // Always show current stack
        println!("\nStack: {:?}", stack);

        match rl.readline("cmd> ") {
            Ok(line) => {
                let input = line.trim();
                match input.split_whitespace().collect::<Vec<_>>().as_slice() {
                    ["push", num] => {
                        if let Ok(n) = num.parse::<i32>() {
                            stack.push(n);
                        } else {
                            println!(" → invalid number: {}", num);
                        }
                    }
                    ["pop"] => {
                        if let Some(v) = stack.pop() {
                            println!(" → popped {}", v);
                        } else {
                            println!(" → stack is empty");
                        }
                    }
                    [""] => { /* ignore empty */ }
                    _ => {
                        println!(" → unknown command: '{}'", input);
                        println!("    Use 'push <n>' or 'pop'");
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                // also break on Ctrl-C / Ctrl-D in rustyline
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    println!("\nExiting.");
    Ok(())
}
