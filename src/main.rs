#![no_std]
#![no_main]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use firefly_rust as ff;
use piccolo::{Closure, Executor, Lua};

#[unsafe(no_mangle)]
extern "C" fn boot() {
    run().ok().unwrap()
}

fn run() -> Result<(), Box<dyn core::error::Error>> {
    let source = r#"
function foo (msg)
    return msg
end

bar = 2 + 3
    "#;

    let mut lua = Lua::empty();
    lua.enter(|ctx| {
        use piccolo::stdlib::*;
        load_base(ctx);
        load_coroutine(ctx);
        load_math(ctx);
        load_string(ctx);
        // load_table(ctx);
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
        let foo = env.get::<_, Closure>(ctx, "foo")?;
        let ex = Executor::start(ctx, foo.into(), "this is my message");
        Ok(ctx.stash(ex))
    })?;

    let result = lua.execute::<String>(&ex)?;
    ff::log_debug(&result);
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
