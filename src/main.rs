// src/main.rs

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use ctrlc;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> rustyline::Result<()> {
    // -- 1. Graceful shutdown on Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    {
        let flag = running.clone();
        ctrlc::set_handler(move || {
            flag.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // -- 2. Initialize line editor and stack
    let mut rl = DefaultEditor::new()?;
    let mut stack: Vec<i32> = Vec::new();

    // -- 3. REPL loop
    while running.load(Ordering::SeqCst) {
        // Always show the current stack
        println!("\nStack: {:?}", stack);

        match rl.readline("cmd> ") {
            Ok(line) => {
                let input = line.trim();
                // record history
                rl.add_history_entry(input).ok();

                // split into command + flags/args
                let parts: Vec<_> = input.split_whitespace().collect();
                let cmd = parts.get(0).map(|s| *s).unwrap_or("");
                let args = &parts[1..];

                match cmd {
                    // push <n>
                    "push" => {
                        if let Some(num) = args.get(0) {
                            match num.parse::<i32>() {
                                Ok(n) => stack.push(n),
                                Err(_) => println!(" → invalid number: {}", num),
                            }
                        } else {
                            println!(" → usage: push <n>");
                        }
                    }

                    // pop [--backwards] [n]
                    "pop" => {
                        // detect flag
                        let backwards = args.contains(&"--backwards");
                        // parse optional count (first non-flag), default = 1
                        let n = args
                            .iter()
                            .find_map(|tok| {
                                if *tok != "--backwards" {
                                    tok.parse::<usize>().ok()
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(1);

                        if backwards {
                            // pop from bottom
                            let cnt = n.min(stack.len());
                            let mut popped = Vec::with_capacity(cnt);
                            for _ in 0..cnt {
                                popped.push(stack.remove(0));
                            }
                            println!(" → popped from bottom: {:?}", popped);
                        } else {
                            // pop from top
                            for _ in 0..n {
                                match stack.pop() {
                                    Some(v) => println!(" → popped {}", v),
                                    None => {
                                        println!(" → stack is empty");
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    // empty line: no-op
                    "" => {}

                    // unknown command
                    _ => {
                        println!(" → unknown command: '{}'", input);
                        println!("    Usage:");
                        println!("      push <n>");
                        println!("      pop [--backwards] [n]");
                    }
                }
            }
            // break on Ctrl-C (handled here) or Ctrl-D
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            // any other error: report and exit
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    println!("\nExiting.");
    Ok(())
}
