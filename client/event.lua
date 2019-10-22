
local function init( logger )
    local e = { listeners = {} }

    function e:register_listener( event, register, callback )
        logger:info( string.format( "%s registered a callback for %s event", register, event ) )
        if not self.listeners[event] then
            self.listeners[event] = {}
        end
        local event_list = self.listeners[event]
        event_list[#event_list+1] = { callback = callback ; register = register }
    end

    function e:trigger( event, options )
        local registered_listeners = self.listeners[event]
        if not registered_listeners then return end
        for _, v in ipairs(registered_listeners) do
            v.callback( options )
        end
    end

    return e
end

return { init = init }
