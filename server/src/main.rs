
mod login;
mod packet_parser;

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


    let zz = "\n";
    let bz = zz.as_bytes();
    println!("!!!!!!!!!!!!!!!");
    println!("blarg {}", bz[0]);

    // TODO this needs to happen in a thread
    // TODO need to set the connect timeout

    login::listen_for_logins();

    Ok(())
}
