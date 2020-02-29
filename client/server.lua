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
