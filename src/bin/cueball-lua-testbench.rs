use cueball::{
    cli::cueball_cli,
    cli::CLIMode,
    cues::{BonkCue, RemarkCue},
    MultitypeCue,
};
use mlua::prelude::*;

fn main() -> Result<(), ()> {
    let lua = Lua::new();
    lua.globals().set("remk_cue", RemarkCue::with_id("0")).unwrap();
    lua.globals().set("bonk_cue", BonkCue::with_id("0")).unwrap();
    lua.globals()
        .set(
            "cuevec",
            vec![
                MultitypeCue::Bonk(BonkCue::with_id("0")),
                MultitypeCue::Remark(RemarkCue::with_id("0")),
            ],
        )
        .unwrap();

    cueball_cli(CLIMode::Lua, lua)
}
