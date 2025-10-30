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

fn run_boot() -> Result<(), anyhow::Error> {
    ff::log_debug("booting...");
    let Some(source) = ff::load_file_buf("main") else {
        anyhow::bail!("file `main` not found");
    };
    let mut lua = Lua::core();
    lua.enter(load_sdk);

    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let closure = Closure::load_with_env(ctx, None, source.data(), env)?;
        let ex = Executor::start(ctx, closure.into(), ());
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&ex)?;

    let maybe_ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let boot = env.get::<_, Closure>(ctx, "boot")?;
        let ex = Executor::start(ctx, boot.into(), ());
        Ok(ctx.stash(ex))
    });
    if let Ok(ex) = maybe_ex {
        lua.execute::<()>(&ex)?;
    }

    #[allow(static_mut_refs)]
    unsafe { LUA.set(lua) }.ok().unwrap();
    ff::log_debug("booted");
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    if let Err(err) = run_update() {
        // TODO: add circuit breaker, exit the app if it keeps failing
        let err = alloc::format!("{err}");
        ff::log_error(&err);
    }
}

fn run_update() -> Result<(), Box<dyn core::error::Error>> {
    let lua = get_lua();
    let maybe_ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let update = env.get::<_, Closure>(ctx, "update")?;
        let ex = Executor::start(ctx, update.into(), ());
        Ok(ctx.stash(ex))
    });
    if let Ok(ex) = maybe_ex {
        lua.execute::<()>(&ex)?;
    }
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    if let Err(err) = run_render() {
        let err = alloc::format!("{err}");
        ff::log_error(&err);
    }
}

fn run_render() -> Result<(), Box<dyn core::error::Error>> {
    let lua = get_lua();
    let maybe_ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let render = env.get::<_, Closure>(ctx, "render")?;
        let ex = Executor::start(ctx, render.into(), ());
        Ok(ctx.stash(ex))
    });
    if let Ok(ex) = maybe_ex {
        lua.execute::<()>(&ex)?;
    }
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn before_exit() {
    if let Err(err) = run_before_exit() {
        let err = alloc::format!("{err}");
        ff::log_error(&err);
    }
}

fn run_before_exit() -> Result<(), Box<dyn core::error::Error>> {
    let lua = get_lua();
    let maybe_ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let before_exit = env.get::<_, Closure>(ctx, "before_exit")?;
        let ex = Executor::start(ctx, before_exit.into(), ());
        Ok(ctx.stash(ex))
    });
    if let Ok(ex) = maybe_ex {
        lua.execute::<()>(&ex)?;
    }
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn handle_menu(idx: i32) {
    if let Err(err) = run_handle_menu(idx) {
        let err = alloc::format!("{err}");
        ff::log_error(&err);
    }
}

fn run_handle_menu(idx: i32) -> Result<(), Box<dyn core::error::Error>> {
    let lua = get_lua();
    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let handle_menu = env.get::<_, Closure>(ctx, "handle_menu")?;
        let ex = Executor::start(ctx, handle_menu.into(), (idx,));
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&ex)?;
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    match run_cheat(cmd, val) {
        Ok(val) => val,
        Err(err) => {
            let err = alloc::format!("{err}");
            ff::log_error(&err);
            0
        }
    }
}

fn run_cheat(cmd: i32, val: i32) -> Result<i32, Box<dyn core::error::Error>> {
    let lua = get_lua();
    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let cheat = env.get::<_, Closure>(ctx, "cheat")?;
        let ex = Executor::start(ctx, cheat.into(), (cmd, val));
        Ok(ctx.stash(ex))
    })?;
    let (res,) = lua.execute::<(i32,)>(&ex)?;
    Ok(res)
}
