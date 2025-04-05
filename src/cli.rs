use log::{debug, error, warn, LevelFilter};
use mlua::prelude::*;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CLIMode {
    CLI,
    Lua,
}

impl std::fmt::Display for CLIMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
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

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    lua.set_warning_function(|_lua, warnstr, _incomplete| Ok(warn!("{}", warnstr)));

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
            Ok(Signal::Success(inp)) => match inp.as_str() {
                "/cli" => mode = CLIMode::CLI,
                "/lua" => mode = CLIMode::Lua,
                "/exit" => break,
                "/help" => {
                    print_help_menu();
                }
                "" => (),
                _ => match mode {
                    CLIMode::CLI => {
                        error!("Unknown CLI command! Try /help");
                    }
                    CLIMode::Lua => match lua.load(inp).eval::<LuaMultiValue>() {
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
                        Err(err) => error!("{}", err),
                    },
                },
            },
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                break;
            }
            x => {
                debug!("Event: {:?}", x);
            }
        }
    }

    Ok(())
}
