
local keyboard_mod = require 'keyboard'
local log_mod = require 'log'
local event_mod = require 'event'

local socket = require 'socket'


-- this only gets called once at the beginning
function love.load()
    s = socket.tcp()
    s:connect( "localhost", 3000 )
    tot, err = s:send( "zap" )

    logger = log_mod.debug_logger()
    event = event_mod.init( logger )
    keyboard = keyboard_mod.init() 
end

-- this function is called continuously
-- dt is the delta time (in seconds) of the last
-- time that the function was called
function love.update(dt)

end

-- this is the only function that the graphics functions
-- will work in
function love.draw()

    if not tot then
        love.graphics.print( "err: " .. err, 40, 40 )
    else
        love.graphics.print( "blah", 20, 40 )
        love.graphics.print( tot, 40, 40 )
    end
 
 --[[

    for _, m in pairs( world:get_mobs() ) do
        love.graphics.print( m.symbol, m.x, m.y )
    end

    local f = love.graphics.newFont();

    love.graphics.print( f:getHeight(), 100, 100 )
    love.graphics.print( f:getWidth('z'), 120, 120 )
    love.graphics.print( 'H', 500, 500 )
    
    love.graphics.circle( 'line', 300, 300, 1, 10 )

    --love.graphics.rectangle( 'line', 500, 500, f:getWidth('H'), f:getHeight() );
    --love.graphics.circle( 'fill', 500, 500, 5, 10 )
    love.graphics.circle( 'line', 500 + f:getWidth('H') / 2, 
                                  500 + f:getHeight() / 2,
                                  f:getHeight() / 3,
                                  10 )
                                  --]]
    
end

function love.mousepressed(x, y, button, istouch)
    
end

function love.mousereleased(x, y, button, istouch)

end

function love.keypressed(key)
    keyboard:keypress( key )
end

function love.keyreleased(key)
    keyboard:keyrelease( key )
end

function love.focus(in_focus)

end

function love.quit()
    logger:close()
end
