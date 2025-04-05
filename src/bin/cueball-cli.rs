use cueball::{cli::cueball_cli, cli::CLIMode};
use mlua::prelude::*;

fn main() -> Result<(), ()> {
    let lua = Lua::new();
    cueball_cli(CLIMode::CLI, lua)
}
