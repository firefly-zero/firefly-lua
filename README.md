# firefly-lua

Lua SDK for making [Firefly Zero](https://fireflyzero.com/) games. Based on a wasm-compatible fork of [piccolo](https://github.com/kyren/piccolo).

* [▶️ getting started](https://docs.fireflyzero.com/dev/getting-started/)
* [🐙 github](https://github.com/firefly-zero/firefly-lua)

## SDK state

TL;DR: Performance is not good yet and only graphics API is implemented.

A Lua app is about 50 times slower than an equivalent [Go](https://github.com/firefly-zero/firefly-go) app. So, until we find a more performant solution, we recommend to use Lua only for simple demos or as an intermediate step when migrating a game from another platform. The latter is a good way to do a gradual migration: first rewrite runtime calls (like calls to draw a shape on the screen) and then change the syntax and code structure to Go.

At the moment, the SDK implements only shape drawing functions and a few helper functions (randomness and logging). This should be sufficient for non-interactive demos, similar to [Fill Patterns](https://catalog.fireflyzero.com/peter.fp8x8).

## Examples

Callbacks:

```lua
function boot()
    -- ...
end

function update()
    -- ...
end

function render()
    -- ...
end
```

Graphics:

```lua
firefly.set_color(firefly.RED, {r=255, g=0, b=0})

firefly.clear_screen(firefly.BLUE)

firefly.draw_point({x=10, y=20}, firefly.BLACK)

firefly.draw_line(
    {x=10, y=20}, {x=30, y=40},
    {color=firefly.RED, width=1},
)

local style = {
    fill_color=firefly.Red,
    stroke_color=firefly.BLUE,
    stroke_width=1,
}
local point = {x=10, y=20}
local size = {width=30, height=40}

firefly.draw_rect(point, size, style)

local corner = {width=4, height=4}
firefly.draw_rounded_rect(point, size, corner, style)

firefly.draw_circle(point, 10, style)

firefly.draw_ellipse(point, size, style)

firefly.draw_triangle(
    {x=40, y=20},
    {x=30, y=40},
    {x=50, y=40},
    style,
)

firefly.draw_arc(
    point,
    10, -- diameter
    0, math.pi / 2, -- start and sweep angles
    style,
)

firefly.draw_sector(
    point,
    10, -- diameter
    0, math.pi / 2, -- start and sweep angles
    style,
)
```

Misc:

```lua
firefly.log_debug("let's go!")

firefly.log_error("oh no...")

firefly.set_seed(13)

local rand_val = firefly.get_random()

firefly.quit()
```
