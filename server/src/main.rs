
mod login;

fn main() -> std::io::Result<()> {

    let zz = "\n";
    let bz = zz.as_bytes();
    println!("!!!!!!!!!!!!!!!");
    println!("blarg {}", bz[0]);

    // TODO this needs to happen in a thread
    // TODO need to set the connect timeout

    login::listen_for_logins();

    Ok(())
}
