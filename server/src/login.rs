
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::time::Duration;
use std::io::{Write, Error, ErrorKind};
use std::thread;
use crate::packet_parser::{Command, self};
use crate::packet_reader;


fn handle_stream( mut stream : TcpStream ) {
    fn f<T, E>( x : Result<T, E> ) -> std::io::Result<T> {
        match x {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::new(ErrorKind::Other, "Error")),
        }
    }
    fn get_register( mut cs : Vec<Command> ) -> std::io::Result<(String, String, String)> {
        if cs.len() != 1 {
            return Err(Error::new(ErrorKind::Other, "Error"));
        }
        match cs.pop().unwrap() {
            Command::Register {id, ip, port} => Ok((id, ip, port)),
            _ => Err(Error::new(ErrorKind::Other, "Error")),
        }
    }

    thread::spawn( move || -> std::io::Result<()> {

        // TODO what timeout value to use?
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
        let packet = packet_reader::read_tcp_packet(&mut stream)?;
        let value = f(std::str::from_utf8(&packet[..(packet.len() - 1)]))?;
        let commands = f(packet_parser::parse(value))?;

        let (id, ip, port) = get_register(commands)?;

        let mut udp = UdpSocket::bind("127.0.0.1:4000")?;
        udp.connect(format!("{}:{}", ip, port))?;

        stream.write(Command::UdpTestReady.to_packet())?;
         
        // wait for response over udp

        stream.write(Command::UdpTestSuccessful.to_packet())?;

        // Send udp message 
        // wait for success over tcp
        // close tcp
        // send udp and id to game thread
    
        Ok(()) 
    } );
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
