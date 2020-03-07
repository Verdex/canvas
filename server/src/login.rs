
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::io::{Read, Error, ErrorKind};
use std::thread;
use crate::packet_parser;

const END_TX : u8 = 10;

fn read_packet<R : Read>( mut stream : R ) -> std::io::Result<Vec<u8>> {

    let mut buffer = [0; 128];
    let mut tot : Vec<[u8; 128]> = vec![];

    loop {
        let count = stream.read(&mut buffer[..])?;

        if count == 0 {
            return Err(Error::new(ErrorKind::Other, "Read zero bytes"));
        }
         
        tot.push( buffer );
        
        if buffer[count - 1] == END_TX {
            break
        }

        buffer = [0; 128];
    }

    let packet : Vec<u8> = tot.into_iter()
                              .map( | b | b.iter()
                                           .fold( vec![], | mut p, sp | { p.push( *sp ); p } ) )
                              .flatten()
                              .collect();


    Ok(packet)
}

fn handle_stream( stream : TcpStream ) {
    fn f<T, E>( x : Result<T, E> ) -> std::io::Result<T> {
        match x {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::new(ErrorKind::Other, "Error")),
        }
    }
    
    thread::spawn( move || -> std::io::Result<()> {

        // TODO what timeout value to use?
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        let packet = read_packet(stream)?;
        let value = f(std::str::from_utf8(&packet[..]))?;
        let commands = f(packet_parser::parse(value))?;
    
        Ok(()) 
    } );


    // TODO if we end up parsing the string here, then do we need a String or can we just use a &str?
}

pub fn listen_for_logins() -> std::io::Result<()> {

    // TODO pass in the ip address and port
    let listener = TcpListener::bind("127.0.0.1:3000")?;


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => {
                println!("encountered error on incoming connection: {}", e); 
                ()
            },
        }
        // TODO we need to check to see if we can stop listening
    }
    Ok(())
}
