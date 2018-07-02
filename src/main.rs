extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::completion::FilenameCompleter;

extern crate rush;

use rush::lex::{lex, LexItem};
//use rush::run::{ShellCmd, Runnable};

fn make_argv(input: &String) -> Result<Vec<String>, String> {
    let lexed = lex(input)?;
    let args: Vec<String> = lexed
        .into_iter()
        .filter_map(|i| match i {
            LexItem::Str(s) => Some(s), _ => None, })
        .collect();
    Ok(args)
}

fn run_cmd(input: &String) {
    use rush::run;
    use rush::run::Cmd;

    let args = match make_argv(input) {
        Ok(a) => a,
        Err(err) => return println!("error: {}", err),
    };
    let (arg_0, argv) = match args.split_first() {
        Some(p) => p,
        None => return println!("no cmd"),
    };
    
    let cmd = Cmd::new(arg_0.to_string(), argv.to_vec());
    if let Err(err) = run::cmd(cmd) {
        println!("error: {}", err);
    }
}

fn main() {
    let mut editor = Editor::<FilenameCompleter>::new();
    editor.set_completer(Some(FilenameCompleter::new()));
    loop {
        let readline = editor.readline(">> ");
        match readline {
            Ok(line) => {
                if !(line.is_empty()) {
                    editor.add_history_entry(&line);
                    run_cmd(&line);
                }
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
}
