--[[

socket.tcp() -> master

master:bind(address, port)
any:close()

master:connect(address, port) -> client
master:listen(backlog) -> server

client:send( string )
client:receive()

server:accept() -> client

any:settimeout( seconds )

==================================

socket.udp() -> unconnected

any:close()

any:receive()
connected:send( string )

connected:setpeername('*') -> unconnected
unconnected:setpeername(address, port) -> connected

unconnected:setsockname(address = '*', port)

============================================

client:
specify server address and port
tcp to server
identify yourself
negotiate a port for udp
test udp both sides send and receive
return success on tcp
wait for success from server on tcp
close tcp

server:
listen for tcp connections
register client identity
negotiate a port for udp
test udp both sides send and receive
wait for success report from client on tcp
return success to client on tcp
close socket

check udp socket for all ready comment
loop if not all ready otherwise stop listening

will need timeout so that any given socket will 
give up without hanging
(although server can probably be written with threads
to sidestep the issue)

--]]

local socket = require "socket"

local function init( log )
    local s = {}

    function s:login( address, port )
        local tcp = socket.tcp()
        tcp:settimeout( 5 ) -- in seconds
        local success, err = tcp:connect( address, port )
        if not success then
            --TODO need workflow that goes back to user
            error( "Encountered error trying to connect to server: " .. err )
        end
        local bytes_sent, err = tcp:send( "i am client" ) 
        if not bytes_sent then
            --TODO need workflow that goes back to user
            error( "Encountered error trying to send to server: " .. err )
        end
        local data, err = tcp:receive()
        if not data then
            --TODO need workflow that goes back to user
            error( "Encountered error trying to receive from server: " .. err )
        end

        tcp:close()
    end

    return s
end


return { init = init
       }
