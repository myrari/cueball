use mlua::prelude::*;
use cueball::{CLIMode, Cue, RemarkCue, cueball_cli};

fn main() -> Result<(), ()> {
    let lua = Lua::new();
    let test_cue: Box<dyn Cue> = Box::new(RemarkCue::default());
    lua.globals().set("test_cue", test_cue).unwrap();

    cueball_cli(CLIMode::Lua, lua)
}
