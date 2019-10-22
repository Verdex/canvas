

local fixedShift = { lshift = true
                   ; rshift = true
                   ; escape = true
                   }

local keyShiftDict = { ['1'] = '!'
                     ; ['2'] = '@'
                     }

local function shift_key( key )
    if fixedShift[key] then return key end
    if string.match( key, '%a' ) then return string.upper( key )
    else return keyShiftDict[key]
    end
end

local function init()
    local k = { left_shift = false
              ; right_shift = false
              ; mode = false
              ; modes = {} 
              }

    function k:set_mode( mode_letter, forwardee )
        self.modes[#self.modes + 1] = { mode = mode_letter
                                      ; forwardee = forwardee 
                                      }
    end

    function k:keypress( key )
        if self.left_shift or self.right_shift then
            key = shift_key( key )
        end

        if key == "escape" then
            if self.mode then
                self.mode:deactivate()
            end
            self.mode = false
        elseif key == "lshift" then 
            self.left_shift = true
        elseif key == "rshift" then
            self.right_shift = true
        elseif not self.mode then
            for _, v in ipairs( self.modes ) do
                if v.mode == key then
                    self.mode = v.forwardee
                    self.mode:activate()
                    break
                end
            end
        else
            self.mode:keypress( key )
        end
    end

    function k:keyrelease( key )
        if self.left_shift or self.right_shift then
            key = shift_key( key )
        end

        if key == "lshift" then 
            self.left_shift = false
        elseif key == "rshift" then
            self.right_shift = false 
        elseif self.mode then
            self.mode:keyrelease( key )
        end
    end

    return k
end

local function init_movement_mode( event, world, hero_id )
    local m = { h_id = false
              ; k_id = false
              ; l_id = false
              ; j_id = false
              }

    function m:keypress(key)
        if key == 'h' then
            self.h_id = world:create_continuous( function ( dt ) 
                event:trigger( "move", { who = hero_id; direction = "west"; time = dt } )
            end )
        elseif key == 'k' then
            self.k_id = world:create_continuous( function ( dt ) 
                event:trigger( "move", { who = hero_id; direction = "north"; time = dt } )
            end )
        elseif key == 'l' then
            self.l_id = world:create_continuous( function ( dt ) 
                event:trigger( "move", { who = hero_id; direction = "east"; time = dt } )
            end )
        elseif key == 'j' then
            self.j_id = world:create_continuous( function ( dt ) 
                event:trigger( "move", { who = hero_id; direction = "south"; time = dt } )
            end )
        end
    end

    function m:keyrelease(key)
        if (key == 'h' or key == 'H') and self.h_id then
            world:destroy_continuous( self.h_id )
            self.h_id = false 
        elseif (key == 'k' or key == 'K') and self.k_id  then
            world:destroy_continuous( self.k_id )
            self.k_id = false 
        elseif (key == 'l' or key == 'L') and self.l_id then
            world:destroy_continuous( self.l_id )
            self.l_id = false 
        elseif (key == 'j' or key == 'J') and self.j_id then
            world:destroy_continuous( self.j_id )
            self.j_id = false 
        end
    end

    function m:activate()
    end

    function m:deactivate()
        if self.h_id then
            world:destroy_continuous( self.h_id )
        end
        if self.k_id then
            world:destroy_continuous( self.k_id )
        end
        if self.l_id then
            world:destroy_continuous( self.l_id )
        end
        if self.j_id then
            world:destroy_continuous( self.j_id )
        end
    end

    return m
end

return { init = init 
       ; init_movement_mode = init_movement_mode
       }

