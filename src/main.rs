#![no_std]
#![no_main]
extern crate alloc;

mod sdk;

use crate::sdk::load_sdk;
use alloc::boxed::Box;
use core::cell::OnceCell;
use firefly_rust as ff;
use piccolo::{Closure, Executor, Lua};

static mut LUA: OnceCell<Lua> = OnceCell::new();

#[unsafe(no_mangle)]
extern "C" fn boot() {
    if let Err(err) = run_boot() {
        let err = alloc::format!("{err}");
        firefly_rust::log_error(&err);
    }
}

fn get_lua() -> &'static mut Lua {
    #[allow(static_mut_refs)]
    unsafe { LUA.get_mut() }.unwrap()
}

fn run_boot() -> Result<(), Box<dyn core::error::Error>> {
    let source = r#"
function boot()
    firefly.log_debug('hello from boot!')
end

function update()
    firefly.log_debug('hello from update!')
end
    "#;

    ff::log_debug("booting...");
    let mut lua = Lua::core();
    lua.enter(load_sdk);

    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let closure = Closure::load_with_env(ctx, None, source.as_bytes(), env)?;
        let ex = Executor::start(ctx, closure.into(), ());
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&ex)?;

    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let boot = env.get::<_, Closure>(ctx, "boot")?;
        let ex = Executor::start(ctx, boot.into(), ());
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&ex)?;

    #[allow(static_mut_refs)]
    unsafe { LUA.set(lua) }.ok().unwrap();
    ff::log_debug("booted");
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    if let Err(err) = run_update() {
        let err = alloc::format!("{err}");
        ff::log_error(&err);
    }
}

fn run_update() -> Result<(), Box<dyn core::error::Error>> {
    let lua = get_lua();
    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let update = env.get::<_, Closure>(ctx, "update")?;
        let ex = Executor::start(ctx, update.into(), ());
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&ex)?;
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    // ...
}
