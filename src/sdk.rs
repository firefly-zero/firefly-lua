use firefly_rust as ff;
use piccolo as pc;

pub fn load_sdk<'gc>(ctx: pc::Context<'gc>) {
    let module = pc::Table::new(&ctx);

    let mut peers = alloc::vec![ff::Peer::COMBINED];
    peers.extend(ff::get_peers().iter());

    // Colors.
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

    // Other constants.
    module.set_field(ctx, "COMBINED", 0);

    // Graphics.

    module.set_field(
        ctx,
        "clear_screen",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let color = pop_color(ctx, &mut stack)?;
            ff::clear_screen(color);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "set_color",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let rgb = pop_rgb(ctx, &mut stack)?;

            let color = pop_color(ctx, &mut stack)?;
            ff::set_color(color, rgb);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_point",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let color = pop_color(ctx, &mut stack)?;
            let point = pop_point(ctx, &mut stack)?;
            ff::draw_point(point, color);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_line",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_line_style(ctx, &mut stack)?;
            let point_b = pop_point(ctx, &mut stack)?;
            let point_a = pop_point(ctx, &mut stack)?;
            ff::draw_line(point_a, point_b, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_rect",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let size = pop_size(ctx, &mut stack)?;
            let point = pop_point(ctx, &mut stack)?;
            ff::draw_rect(point, size, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_rounded_rect",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let corner = pop_size(ctx, &mut stack)?;
            let size = pop_size(ctx, &mut stack)?;
            let point = pop_point(ctx, &mut stack)?;
            ff::draw_rounded_rect(point, size, corner, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_circle",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let d = stack.from_back::<i32>(ctx)?;
            let point = pop_point(ctx, &mut stack)?;
            ff::draw_circle(point, d, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_ellipse",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let size = pop_size(ctx, &mut stack)?;
            let point = pop_point(ctx, &mut stack)?;
            ff::draw_ellipse(point, size, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_triangle",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let point_c = pop_point(ctx, &mut stack)?;
            let point_b = pop_point(ctx, &mut stack)?;
            let point_a = pop_point(ctx, &mut stack)?;
            ff::draw_triangle(point_a, point_b, point_c, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_arc",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let sweep = pop_angle(ctx, &mut stack)?;
            let start = pop_angle(ctx, &mut stack)?;
            let d = stack.from_back::<i32>(ctx)?;
            let p = pop_point(ctx, &mut stack)?;
            ff::draw_arc(p, d, start, sweep, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "draw_sector",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let style = pop_style(ctx, &mut stack)?;
            let sweep = pop_angle(ctx, &mut stack)?;
            let start = pop_angle(ctx, &mut stack)?;
            let d = stack.from_back::<i32>(ctx)?;
            let p = pop_point(ctx, &mut stack)?;
            ff::draw_sector(p, d, start, sweep, style);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    // Input.

    module.set_field(
        ctx,
        "read_pad",
        pc::Callback::from_fn(&ctx, move |ctx, _, mut stack| {
            let peer = stack.consume::<u8>(ctx)?;
            let Some(peer) = peers.get(peer as usize) else {
                return format_error("invalid peer");
            };
            let Some(pad) = ff::read_pad(*peer) else {
                stack.push_back(pc::Value::Nil);
                return Ok(pc::CallbackReturn::Return);
            };

            let res = pc::Table::new(&ctx);
            res.set(ctx, "x", pad.x)?;
            res.set(ctx, "y", pad.y)?;
            stack.push_back(pc::Value::Table(res));
            Ok(pc::CallbackReturn::Return)
        }),
    );

    // Misc.

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
        "set_seed",
        pc::Callback::from_fn(&ctx, |ctx, _, mut stack| {
            let seed = stack.from_back::<u32>(ctx)?;
            ff::set_seed(seed);
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "get_random",
        pc::Callback::from_fn(&ctx, |_, _, mut stack| {
            let res = ff::get_random();
            stack.push_back(pc::Value::Integer(i64::from(res)));
            Ok(pc::CallbackReturn::Return)
        }),
    );

    module.set_field(
        ctx,
        "quit",
        pc::Callback::from_fn(&ctx, |_, _, _| {
            ff::quit();
            Ok(pc::CallbackReturn::Return)
        }),
    );

    ctx.set_global("firefly", module);
}

fn pop_rgb<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::RGB, piccolo::Error<'gc>> {
    let rgb = stack.from_back::<pc::Table>(ctx)?;
    let r = rgb.get::<_, u8>(ctx, "r")?;
    let g = rgb.get::<_, u8>(ctx, "g")?;
    let b = rgb.get::<_, u8>(ctx, "b")?;
    let rgb = ff::RGB { r, g, b };
    Ok(rgb)
}

fn pop_point<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::Point, piccolo::Error<'gc>> {
    let point = stack.from_back::<pc::Table>(ctx)?;
    let x = point.get::<_, i32>(ctx, "x")?;
    let y = point.get::<_, i32>(ctx, "y")?;
    let point = ff::Point { x, y };
    Ok(point)
}

fn pop_color<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::Color, piccolo::Error<'gc>> {
    let color = stack.from_back::<i64>(ctx)?;
    let Ok(color) = ff::Color::try_from(color as usize) else {
        return format_error("invalid color");
    };
    Ok(color)
}

fn pop_line_style<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::LineStyle, piccolo::Error<'gc>> {
    let style = stack.from_back::<pc::Table>(ctx)?;
    let color = style.get::<_, i64>(ctx, "color")?;
    let Ok(color) = ff::Color::try_from(color as usize) else {
        return format_error("invalid color");
    };
    let width = style.get::<_, i32>(ctx, "width")?;
    let style = ff::LineStyle::new(color, width);
    Ok(style)
}

fn pop_style<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::Style, piccolo::Error<'gc>> {
    let style = stack.from_back::<pc::Table>(ctx)?;

    let fill_color = style.get::<_, i64>(ctx, "fill_color")?;
    let Ok(fill_color) = ff::Color::try_from(fill_color as usize) else {
        return format_error("invalid fill_color");
    };

    let stroke_color = style.get::<_, i64>(ctx, "stroke_color")?;
    let Ok(stroke_color) = ff::Color::try_from(stroke_color as usize) else {
        return format_error("invalid stroke_color");
    };

    let stroke_width = style.get::<_, i32>(ctx, "stroke_width")?;
    let style = ff::Style {
        fill_color,
        stroke_color,
        stroke_width,
    };
    Ok(style)
}

fn pop_size<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::Size, piccolo::Error<'gc>> {
    let style = stack.from_back::<pc::Table>(ctx)?;
    let width = style.get::<_, i32>(ctx, "width")?;
    let height = style.get::<_, i32>(ctx, "height")?;
    let style = ff::Size { width, height };
    Ok(style)
}

fn pop_angle<'gc>(
    ctx: piccolo::Context<'gc>,
    stack: &mut piccolo::Stack<'gc, '_>,
) -> Result<ff::Angle, piccolo::Error<'gc>> {
    let angle = stack.from_back::<f32>(ctx)?;
    let angle = ff::Angle::from_radians(angle);
    Ok(angle)
}

fn format_error<'gc, T>(err: &'static str) -> Result<T, pc::Error<'gc>> {
    let err = anyhow::anyhow!(err);
    let err = alloc::sync::Arc::new(err);
    let err = pc::RuntimeError(err);
    let err = pc::Error::Runtime(err);
    Err(err)
}
