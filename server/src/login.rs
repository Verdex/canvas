
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::time::Duration;
use std::io::{Write, Error, ErrorKind};
use std::thread;
use crate::packet_parser::{Command, self};
use crate::packet_reader;

fn f<T, E>( x : Result<T, E> ) -> std::io::Result<T> {
    match x {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::new(ErrorKind::Other, "Error")),
    }
}

fn read_tcp_command( stream : &mut TcpStream ) -> std::io::Result<Vec<Command>> {

    let packet = packet_reader::read_tcp_packet(stream)?;
    let string = f(std::str::from_utf8(&packet[..(packet.len() - 1)]))?;
    let commands = f(packet_parser::parse(string))?;

    Ok(commands) 
}

fn read_udp_command( socket : &mut UdpSocket ) -> std::io::Result<Vec<Command>> {

    let packet = packet_reader::read_udp_packet(socket)?;
    let string = f(std::str::from_utf8(&packet[..(packet.len() - 1)]))?;
    let commands = f(packet_parser::parse(string))?;

    Ok(commands) 
}

fn handle_stream( mut stream : TcpStream ) {
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
        let register_command = read_tcp_command( &mut stream )?;

        let (id, ip, port) = get_register(register_command)?;

        let mut udp = UdpSocket::bind("127.0.0.1:4000")?;
        udp.connect(format!("{}:{}", ip, port))?;
        udp.set_read_timeout(Some(Duration::from_secs(2)))?;
        udp.set_write_timeout(Some(Duration::from_secs(2)))?;

        stream.write(Command::UdpTestReady.to_packet())?;
        
        let udp_test_command = read_udp_command( &mut udp )?;

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
