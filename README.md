# firefly-lua

## Examples

Graphics:

```lua
firefly.set_color(firefly.RED, {r=255, g=0, b=0})

firefly.clear_screen(firefly.BLUE)

firefly.draw_point({x=10, y=20}, firefly.BLACK)

firefly.draw_line(
    {x=10, y=20}, {x=30, y=40},
    {color=firefly.RED, width=1}
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
```

Misc:

```lua
firefly.log_debug("let's go!")

firefly.log_error("oh no...")
```
