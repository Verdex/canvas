
-module(connector).
-export( [start/0] ).

-define(PORT, 3000).


start() -> 


    {ok, LSocket} = gen_tcp:listen(?PORT, [binary, 
                                          {packet, 0}, 
                                          {active, false}]),


    {ok, Socket} = gen_tcp:accept(LSocket),


    {ok, Bin} = gen_tcp:recv(Socket, 3),

    io:fwrite( "::~s", [binary_to_list( Bin )] ),

    gen_tcp:send(Socket, "data").

