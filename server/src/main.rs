
use std::io::Read;
use std::net::{TcpListener, TcpStream};

const END_TX : u8 = 10;


fn read_packet( mut stream : TcpStream ) -> std::io::Result<()> {

    let mut buffer = [0; 512];
    let mut tot : Vec<[u8; 512]> = vec![];

    loop {
        let count = stream.read(&mut buffer[..])?;

        if count == 0 {
            // return error
        }
         
        tot.push( buffer );
        
        if buffer[count - 1] == END_TX {
            break
        }

        buffer = [0; 512];
    }

    // TODO make sure we keep reading in case the total length is longer than 512
    Ok(())
}

fn handle_stream( mut stream : TcpStream ) -> std::io::Result<()> {
    // TODO this needs to happen in a thread
    // TODO need a timeout so we can kill the thread if it turns out nothing is coming


    /*let x = String::from_utf8( buffer[..count].to_vec() ); 
    // TODO if we end up parsing the string here, then do we need a String or can we just use a &str?
    
    match x {
        Err(e) => panic!("Bad String : {}", e),
        Ok( v ) => println!("got : {}", v),
    }*/

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
