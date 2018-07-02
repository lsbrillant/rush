pub trait Runnable {
    fn run(self) -> Result<(), String>;
}


use std::process::Command;
pub struct ShellCmd {
    cmd: Box<Command>
}

#[allow(dead_code)]
pub struct Cmd {
    pub program: String,
    pub args: Vec<String>
}

impl Cmd {
    pub fn new(program: String, args: Vec<String>) -> Cmd {
        Cmd {program, args}
    }
}


#[allow(dead_code)]
pub fn cmd(cmd: Cmd) -> Result<(), String> {
    match cmd.program.as_ref() {
        "cd" => { 
            use std::env;
            use std::path::Path;
            let path = if !(cmd.args.len() > 0) {
                env::var("HOME").unwrap_or("/".to_string())
            } else {
                cmd.args[0].clone()
            };
            if let Err(err) = env::set_current_dir(Path::new(&path)) {
                return Err(format!("{}", err))
            }
        } 
        _ => {         
            let mut sh_cmd = ShellCmd::new(cmd.program, cmd.args);
            if let Err(err) = sh_cmd.run() {
                return Err(format!("{}", err))
            };
        }
    }
    Ok(())
}

use std::ffi::OsStr;
impl ShellCmd {
    pub fn new<I, S>(program: S, args: I) -> ShellCmd
        where 
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        ShellCmd{ cmd:Box::new(cmd) }
    }
}

impl Runnable for ShellCmd {
    fn run(mut self) -> Result<(), String> {
        match self.cmd.spawn() {
            Ok(mut child) => { 
                child.wait().expect("couldn't wait");
            }
            Err(err) => return Err(format!("{}", err).to_string()) 
        }
        Ok(())
    }
}

