use crate::cue_trait::CLIMode;
use mlua::prelude::*;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

fn print_help_menu() {
    println!("Help menu:");

    println!("\nDefault Commands:");
    println!("/help\t\tDisplay this menu");
    println!("/exit\t\tExit Cueball");
    println!("/cli\t\tSwitch to CLI mode");
    println!("/lua\t\tSwitch to lua interpreter mode");
}

pub fn cueball_cli(initial_mode: CLIMode, lua: Lua) -> Result<(), ()> {
    let mut line_editor_cli = Reedline::create();
    let mut line_editor_lua = Reedline::create();
    let mut mode = initial_mode;

    loop {
        let line_editor = match mode {
            CLIMode::CLI => &mut line_editor_cli,
            CLIMode::Lua => &mut line_editor_lua,
        };
        let prompt = DefaultPrompt::new(
            DefaultPromptSegment::Basic(format!("Cueball {}", mode.to_string())),
            DefaultPromptSegment::CurrentDateTime,
        );
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(inp)) => {
                match inp.as_str() {
                    "/cli" => mode = CLIMode::CLI,
                    "/lua" => mode = CLIMode::Lua,
                    "/exit" => break,
                    "/help" => {
                        print_help_menu();
                    }
                    "" => (),
                    _ => match mode {
                        CLIMode::CLI => {
                            println!("Unknown CLI command! Try /help");
                        }
                        CLIMode::Lua => {
                            match lua.load(inp).eval::<LuaMultiValue>() {
                                Ok(xs) => {
                                    if !xs.is_empty() {
                                        println!(
                                            "{}",
                                            xs.iter()
                                                .map(|x| format!("{:#?}", x))
                                                .collect::<Vec<_>>()
                                                .join("\t")
                                        )
                                    }
                                }
                                // Switch to Log
                                Err(err) => println!("Error: {:?}", err),
                            }
                        }
                    },
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
