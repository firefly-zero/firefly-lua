use firefly_rust as ff;
use piccolo as pc;

pub fn load_sdk<'gc>(ctx: pc::Context<'gc>) {
    let module = pc::Table::new(&ctx);

    module.set_field(ctx, "NONE", 0);
    module.set_field(ctx, "BLACK", 1);
    module.set_field(ctx, "PURPLE", 2);
    module.set_field(ctx, "RED", 3);
    module.set_field(ctx, "ORANGE", 4);
    module.set_field(ctx, "YELLOW", 5);
    module.set_field(ctx, "LIGHT_GREEN", 6);
    module.set_field(ctx, "GREEN", 7);
    module.set_field(ctx, "DARK_GREEN", 8);
    module.set_field(ctx, "DARK_BLUE", 9);
    module.set_field(ctx, "BLUE", 10);
    module.set_field(ctx, "LIGHT_BLUE", 11);
    module.set_field(ctx, "CYAN", 12);
    module.set_field(ctx, "WHITE", 13);
    module.set_field(ctx, "LIGHT_GRAY", 14);
    module.set_field(ctx, "GRAY", 15);
    module.set_field(ctx, "DARK_GRAY", 16);

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
            let color = stack.consume::<i64>(ctx)?;
            let Ok(color) = ff::Color::try_from(color as usize) else {
                return format_error("invalid color");
            };
            ff::clear_screen(color);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "set_color",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let rgb = stack.from_back::<pc::Table>(ctx)?;
            let r = rgb.get::<_, u8>(ctx, "r")?;
            let g = rgb.get::<_, u8>(ctx, "g")?;
            let b = rgb.get::<_, u8>(ctx, "b")?;
            let rgb = ff::RGB { r, g, b };

            let color = stack.from_back::<i64>(ctx)?;
            let Ok(color) = ff::Color::try_from(color as usize) else {
                return format_error("invalid color");
            };

            ff::set_color(color, rgb);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_point",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let color = stack.from_back::<i64>(ctx)?;
            let Ok(color) = ff::Color::try_from(color as usize) else {
                return format_error("invalid color");
            };

            let point = stack.from_back::<pc::Table>(ctx)?;
            let x = point.get::<_, i32>(ctx, "x")?;
            let y = point.get::<_, i32>(ctx, "y")?;
            let point = ff::Point { x, y };

            ff::draw_point(point, color);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_line",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = stack.from_back::<pc::Table>(ctx)?;
            let color = style.get::<_, i64>(ctx, "color")?;
            let Ok(color) = ff::Color::try_from(color as usize) else {
                return format_error("invalid color");
            };
            let width = style.get::<_, i32>(ctx, "width")?;
            let style = ff::LineStyle::new(color, width);

            let point_b = stack.from_back::<pc::Table>(ctx)?;
            let x = point_b.get::<_, i32>(ctx, "x")?;
            let y = point_b.get::<_, i32>(ctx, "y")?;
            let point_b = ff::Point { x, y };

            let point_a = stack.from_back::<pc::Table>(ctx)?;
            let x = point_a.get::<_, i32>(ctx, "x")?;
            let y = point_a.get::<_, i32>(ctx, "y")?;
            let point_a = ff::Point { x, y };

            ff::draw_line(point_a, point_b, style);
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
