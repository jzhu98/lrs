extern crate rustyline;
extern crate telescope;

use telescope::parse;
use rustyline::Editor;
use rustyline::error::ReadlineError as RLError;

fn main() {
    // Prompt constants
    let header = format!("telescope v{}\n---------", env!("CARGO_PKG_VERSION"));
    let prompt = "> ";

    let mut rl = Editor::<()>::new();

    println!("{}", header);

    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line == "exit" || line == "quit" {
                    break;
                }
                let _ = parse::parse_Lang(&line)
                    .map_err(|e| e.into())
                    .and_then(|x| x.eval())
                    .map(|v| println!("{}", v))
                    .map_err(|e| println!("{}", e));
            },
            Err(RLError::Interrupted) | Err(RLError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        };
    }
}
