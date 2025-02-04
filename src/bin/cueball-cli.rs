use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use mlua::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum CLIMode {
    CLI,
    Lua
}
impl std::fmt::Display for CLIMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<(), ()> {
    let mut line_editor_cli = Reedline::create();
    let mut line_editor_lua = Reedline::create();
    let mut mode = CLIMode::CLI;

    let lua = Lua::new();

    loop {
        let line_editor = match mode {
            CLIMode::CLI => &mut line_editor_cli,
            CLIMode::Lua => &mut line_editor_lua
        };
        let prompt = DefaultPrompt::new(
            DefaultPromptSegment::Basic(format!("Cueball {}",
                mode.to_string())),
            DefaultPromptSegment::CurrentDateTime);
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(inp)) => {
                match inp.as_str() {
                    "/cli" => mode = CLIMode::CLI,
                    "/lua" => mode = CLIMode::Lua,
                    "/exit" => break,
                    ""      => (),
                    _ => match mode {
                        CLIMode::CLI => (),
                        CLIMode::Lua => {
                            let torun = match inp.chars().nth(0).unwrap() {
                                '=' => format!("print({})", inp.split_at(1).1),
                                _ => inp
                            };
                            match lua.load(torun).exec() {
                                Ok(()) => (),
                                // Switch to Log
                                Err(err) => println!("Error: {:?}", err)
                            }
                        }
                    }
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }

    Ok(())
}
