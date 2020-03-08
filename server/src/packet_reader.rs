
use std::net::{UdpSocket};
use std::io::{Read, Error, ErrorKind};

const END_TX : u8 = 10;

pub fn read_tcp_packet<R : Read>( stream : &mut R ) -> std::io::Result<Vec<u8>> {

    let mut buffer = [0; 128];
    let mut tot : Vec<[u8; 128]> = vec![];

    loop {
        let count = stream.read(&mut buffer)?;

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

pub fn read_udp_packet( socket : &mut UdpSocket ) -> std::io::Result<Vec<u8>> {
    
    let mut buffer = [0; 128];
    let mut tot : Vec<[u8; 128]> = vec![];

    loop {
        let count = socket.recv(&mut buffer)?;

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
