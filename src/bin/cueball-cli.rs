use mlua::prelude::*;
use cueball::{CLIMode, cueball_cli};

fn main() -> Result<(), ()> {
    let lua = Lua::new();
    cueball_cli(CLIMode::CLI, lua)
}
