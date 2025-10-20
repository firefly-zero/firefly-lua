use firefly_rust as ff;
use piccolo as pc;

pub fn load_sdk<'gc>(ctx: pc::Context<'gc>) {
    let module = pc::Table::new(&ctx);

    module.set_field(
        ctx,
        "log_debug",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let text = stack.consume::<pc::String>(ctx)?;
            let text = text.as_bytes();
            let text = unsafe { alloc::str::from_utf8_unchecked(text) };
            ff::log_debug(text);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "log_error",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let text = stack.consume::<pc::String>(ctx)?;
            let text = text.as_bytes();
            let text = unsafe { alloc::str::from_utf8_unchecked(text) };
            ff::log_error(text);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "clear_screen",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let color = stack.consume::<u8>(ctx)?;
            let Ok(color) = ff::Color::try_from(color as usize) else {
                return format_error("invalid color");
            };
            ff::clear_screen(color);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    ctx.set_global("firefly", module);
}

fn format_error<'gc, T>(err: &'static str) -> Result<T, pc::Error<'gc>> {
    let err = anyhow::anyhow!(err);
    let err = alloc::sync::Arc::new(err);
    let err = pc::RuntimeError(err);
    let err = pc::Error::Runtime(err);
    Err(err)
}
