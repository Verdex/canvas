
use std::str::Chars;
use std::iter::Peekable;

pub enum Packet {

}


// [<packet_type>|atom:<param>|list:<length>:<param1>:<param2>]

struct Parser<'a> {
    orig : Peekable<Chars<'a>>,
    done : bool
}

impl<'a> Parser<'a> {
    fn is(&mut self, c : char) -> Result<(), String> {
        match self.orig.peek() {
            Some(value) if *value == c => Ok(()),
            Some(value) =>  Err(format!("Expected {} but found {}", c, *value)),
            None => Err(format!("Expected {} but found Nothing", c)),
        }
    }

    fn next(&mut self) {
        match self.orig.next() {
            None => self.done = true,
            _ => (),
        }
    }
}


pub fn parse( packet : &str ) -> Result<Packet, String> {
    let mut parser = Parser{ orig : packet.chars().peekable(), done : false };

    parser.is( '[' )?;
    parser.next();
     
     Err("Unknown Failure".to_string())
}
