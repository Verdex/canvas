
local function mob_id()
    return { t = "mob" }
end

local function continuous_id()
    return { t = "continuous" }
end

local function init() 
    local w = { mobs = {}
              ; continuous = {}
              }
    function w:get_mob( id )
        assert( id.t == "mob", "encountered non mob id in get_mob : " .. id.t )
        return self.mobs[id] or false 
    end

    function w:create_mob( mob )
        local id = mob_id()
        mob.id = id
        self.mobs[id] = mob
        return id
    end

    function w:get_mobs()
        return self.mobs
    end

    function w:create_continuous( c )
        local id = continuous_id()
        self.continuous[id] = { id = id, fn = c }
        return id
    end

    function w:destroy_continuous( id )
        assert( id.t == "continuous", "encountered non continuous id in destroy_continuous: " .. id.t )
        self.continuous[id] = nil
    end

    function w:get_continuous()
        return self.continuous
    end

    return w
end


return { init = init }
