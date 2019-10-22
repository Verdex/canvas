

local function debug_logger() 
    local l = { f = io.open('logs/debug.log', 'a') }

    function l:info( message )
        local d = os.date()
        self.f:write( "[INFO] : " .. d .. " : " .. message .. '\n') 
    end

    function l:err( message )
        local d = os.date()
        self.f:write( "[ERROR] : " .. d .. " : " .. message .. '\n' )
    end

    function l:tag( tag, message )
        local d = os.date()
        self.f:write( "[" .. tag .. "] : " .. d .. " : " .. message .. '\n' )
    end

    function l:special( file, message )
        local f = io.open('logs/'..file, 'a')
        local d = os.date()
        f:write( d .. " : " .. message .. "\n" )
        f:close()
    end

    function l:close()
        local d = os.date()
        self.f:write( d .. " : CLOSE LOGGER\n" ) 
        self.f:close()
    end

    return l
end


return { debug_logger = debug_logger
       ;
       }
