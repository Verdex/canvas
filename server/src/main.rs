
mod login;
mod packet_parser;
mod packet_reader;

fn main() -> std::io::Result<()> {

/*
    use std::thread;
    let child = thread::spawn( move || ... );

    let result = child.join(); 


    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    tx.send(val).unwrap();

    rx.try_recv() // returns result<t,e> .  Error when there are no messages
    rx.recv() // blocks


    mpsc::Sender::clone(&tx)
*/

    
    // TODO need to set the connect timeout

    let _ = login::listen_for_logins();
    let _ = packet_parser::parse("[blah]");

    Ok(())
}
