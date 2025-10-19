use firefly_rust::*;
use piccolo::*;

pub fn load_sdk<'gc>(ctx: Context<'gc>) {
    let module = Table::new(&ctx);

    module.set_field(
        ctx,
        "log_debug",
        Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let text = stack.consume::<String>(ctx)?;
            let text = text.as_bytes();
            let text = unsafe { alloc::str::from_utf8_unchecked(text) };
            log_debug(text);
            Ok(CallbackReturn::Return)
        }),
    );

    ctx.set_global("firefly", module);
}
