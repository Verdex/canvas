

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

return { init = init 
       }

