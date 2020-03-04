
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::io::{Read, Error, ErrorKind};

const END_TX : u8 = 10;


fn read_packet<R : Read>( mut stream : R ) -> std::io::Result<Vec<u8>> {

    let mut buffer = [0; 512];
    let mut tot : Vec<[u8; 512]> = vec![];

    loop {
        let count = stream.read(&mut buffer[..])?;

        if count == 0 {
            return Err(Error::new(ErrorKind::Other, "Read zero bytes"));
        }
         
        tot.push( buffer );
        
        if buffer[count - 1] == END_TX {
            break
        }

        buffer = [0; 512];
    }

    let packet : Vec<u8> = tot.into_iter()
                              .map( | b | b.iter()
                                           .fold( vec![], | mut p, sp | { p.push( *sp ); p } ) )
                              .flatten()
                              .collect();


    Ok(packet)
}

fn handle_stream( mut stream : TcpStream ) -> std::io::Result<()> {
    // TODO this needs to happen in a thread
    // TODO need a timeout so we can kill the thread if it turns out nothing is coming
    
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    let packet = read_packet(stream);
    match packet {
        Ok(x) => println!( "ok " ),
        Err(e) => println!( "err : {}", e ),
    }
    
    

    //let x = String::from_utf8( buffer[..count].to_vec() ); 
    // TODO if we end up parsing the string here, then do we need a String or can we just use a &str?
    

    Ok(())
}

fn main() -> std::io::Result<()> {

    let zz = "\n";
    let bz = zz.as_bytes();
    println!("!!!!!!!!!!!!!!!");
    println!("blarg {}", bz[0]);

    // TODO this needs to happen in a thread
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream)?,
            Err(e) => {
                println!("encountered error on incoming connection: {}", e); 
                ()
            },
        }
        // TODO we need to check to see if we can stop listening
    }

    Ok(())
}
