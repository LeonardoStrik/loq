use std::io;
use std::io::Write;
use std::vec::Vec;

fn repl() -> io::Result<()> {
    let mut buffer = String::new();
    let mut input = String::new();
    let mut history: Vec<String> = Vec::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut quit = false;

    while !quit {
        input.clear();
        print!(">  ");
        stdout.flush()?;
        stdin.read_line(&mut input)?;
        input.truncate(input.len() - 2);
        history.push(input.clone());
        match input.as_str() {
            "quit;" | "q;" => break,
            "clear;" | "c;" => {
                buffer.clear();
                history.clear();
            }
            "undo;" | "u;" => {
                let _ = history.pop();
                match history.pop() {
                    Some(x) => buffer.truncate(buffer.len() - x.len()),
                    None => println!("ERROR: Nothing to undo!"),
                };
            }
            _ => {
                buffer.push_str(input.as_str());
            }
        };
        if !buffer.is_empty() {
            println!("  =>  {}", buffer);
        };
    }
    Ok(())
}
