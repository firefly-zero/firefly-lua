#![no_std]
#![no_main]
use firefly_rust as ff;
use mlua::prelude::*;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    run().ok().unwrap()
}

fn run() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;

    lua.load("for k,v in pairs(map_table) do print(k,v) end")
        .exec()?;

    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    // ...
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    // ...
}
