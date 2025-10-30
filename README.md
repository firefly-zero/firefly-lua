# firefly-lua

Lua SDK for making [Firefly Zero](https://fireflyzero.com/) games. Based on a wasm-compatible fork of [piccolo](https://github.com/kyren/piccolo).

* [▶️ getting started](https://docs.fireflyzero.com/dev/getting-started/)
* [🐙 github](https://github.com/firefly-zero/firefly-lua)

## SDK state

A Lua app is about 50 times slower than an equivalent [Go](https://github.com/firefly-zero/firefly-go) app. So, until we find a more performant solution, we recommend to use Lua only for simple demos or as an intermediate step when migrating a game from another platform. The latter is a good way to do a gradual migration: first rewrite runtime calls (like calls to draw a shape on the screen) and then change the syntax and code structure to Go.

At the moment, the SDK doesn't implement audio API.

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

function before_exit()
    -- ...
end

function handle_menu(index)
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

local font = firefly.load_file("font")
firefly.draw_text("oh hi mark", font, point, firefly.BLACK)

firefly.draw_qr(
    "https://fireflyzero.com/",
    point, firefly.BLACK, firefly.WHITE,
)
```

Multiplayer:

```lua
local peer = firefly.get_me()

local peers = firefly.get_peers()
for i = 1, #peers do
    peer = peers[i]
end
```

Input:

```lua
local pad = firefly.read_pad(peer)
if pad then
    {pad.x, pad.y}
end

local btns = firefly.read_buttons(peer)
{btns.s, btns.e, btns.w, btns.n}

-- Read combined inputs.
pad = firefly.read_pad(firefly.COMBINED)
btns = firefly.read_buttons(firefly.COMBINED)
```

Filesystem:

```lua
local file = firefly.load_file("font")

firefly.dump_file("font", file)

firefly.remove_file("font")
```

Menu:

```lua
firefly.add_menu_item(1, "intentory")
firefly.remove_menu_item(1)
firefly.open_menu()
```

Boards (scoreboards) and badges (achievements):

```lua
local badge1 = 1
local badge2 = 2

local progress = firefly.get_progress(peer, badge1)
{progress.done, progress.goal}

progress = firefly.add_progress(peer, badge1, 1)

local board1 = 1
local board2 = 2

local best_score = firefly.get_score(peer, board1)
best_score = firefly.add_score(peer, board1, 10)
```

Misc:

```lua
firefly.log_debug("let's go!")

firefly.log_error("oh no...")

firefly.set_seed(13)

local rand_val = firefly.get_random()

firefly.quit()
```
