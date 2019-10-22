
local keyboard_mod = require 'keyboard'
local log_mod = require 'log'
local world_mod = require 'world'
local event_mod = require 'event'

local socket = require 'socket'


-- this only gets called once at the beginning
function love.load()
    s = socket.tcp()
    s:connect( "localhost", 3000 )

    logger = log_mod.debug_logger()
    world = world_mod.init()
    event = event_mod.init( logger )
    keyboard = keyboard_mod.init() 

    hero_id = world:create_mob( { x = 10 ; y = 10 ; move = 10 ;
                                  symbol = '@' } )
    
    world:create_mob( { x = 50 ; y = 25 ; move = 1 ; symbol = 'Z' } )

    local movement_mode = keyboard_mod.init_movement_mode( event, world, hero_id )

    keyboard:set_mode( 'm', movement_mode )
end

-- this function is called continuously
-- dt is the delta time (in seconds) of the last
-- time that the function was called
function love.update(dt)
    for _, c in pairs( world:get_continuous() ) do
        c.fn( dt ) 
    end
end

-- this is the only function that the graphics functions
-- will work in
function love.draw()

    local tot, err = s:send( "zap" )

    if not tot then
        love.graphics.print( err, 40, 40 )
    else
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
