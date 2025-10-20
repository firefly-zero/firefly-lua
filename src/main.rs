#![no_std]
#![no_main]
extern crate alloc;

mod sdk;

use alloc::boxed::Box;
use firefly_rust as ff;
use piccolo::{Closure, Executor, Lua};

use crate::sdk::load_sdk;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    run().ok().unwrap()
}

fn run() -> Result<(), Box<dyn core::error::Error>> {
    let source = r#"
function boot()
    firefly.log_debug('hello world!')
end
    "#;

    let mut lua = Lua::empty();
    lua.enter(|ctx| {
        use piccolo::stdlib::*;
        load_base(ctx);
        load_coroutine(ctx);
        load_math(ctx);
        load_string(ctx);
        load_sdk(ctx);
        load_table(ctx);
    });

    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let closure = Closure::load_with_env(ctx, None, source.as_bytes(), env)?;
        let ex = Executor::start(ctx, closure.into(), ());
        Ok(ctx.stash(ex))
    })?;

    lua.execute::<()>(&ex)?;

    let ex = lua.try_enter(|ctx| {
        let env = ctx.globals();
        // env.set(ctx, key, value)
        let boot = env.get::<_, Closure>(ctx, "boot")?;
        let ex = Executor::start(ctx, boot.into(), ());
        Ok(ctx.stash(ex))
    })?;

    lua.execute::<()>(&ex)?;
    ff::log_debug("boot done");
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
