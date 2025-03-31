use cueball::{cueball_cli, CLIMode, RemarkCue, BonkCue, MultitypeCue};
use mlua::prelude::*;

fn main() -> Result<(), ()> {
    let lua = Lua::new();
    lua.globals().set("remk_cue", RemarkCue::default()).unwrap();
    lua.globals().set("bonk_cue", BonkCue::default()).unwrap();
    lua.globals().set("cuevec", vec![MultitypeCue::Bonk(BonkCue::default()),
        MultitypeCue::Remark(RemarkCue::default())]).unwrap();

    cueball_cli(CLIMode::Lua, lua)
}
