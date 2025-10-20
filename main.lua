i = 1

function boot()
    firefly.log_debug('hello from boot!')
end

function update()
    i = i + 1
    if i > 16 then
      i = 1
    end
end

function render()
    firefly.clear_screen(i)
end
