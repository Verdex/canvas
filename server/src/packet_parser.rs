
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

    fn symbol(&mut self) -> Result<String, String> {
        fn f<T>( v : Option<T> ) -> Result<T, String> {
            match v {
                Some(v) => Ok(v),
                None => Err("Encountered end of stream in symbol".to_string()),
            }
        }

        let mut ret = vec![];
        let mut c = *f(self.orig.peek())?;
        while c.is_alphanumeric() {
            self.orig.next();
            ret.push(c);
            c = *f(self.orig.peek())?; 
        }
        Ok(ret.into_iter().collect())
    }
}

pub fn parse( packet : &str ) -> Result<Packet, String> {
    let mut parser = Parser{ orig : packet.chars().peekable(), done : false };

    parser.is( '[' )?;
    parser.next();
    let packet_type = parser.symbol()?;
     
    Err("Unknown Failure".to_string())
}
