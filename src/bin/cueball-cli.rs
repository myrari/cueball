use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

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
    let mut line_editor = Reedline::create();
    let mut mode = CLIMode::CLI;

    loop {
        let prompt = DefaultPrompt::new(
            DefaultPromptSegment::Basic(format!("Cueball {}",
                mode.to_string())),
            DefaultPromptSegment::CurrentDateTime);
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                println!("{}", buffer);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }

    Ok(())
}
