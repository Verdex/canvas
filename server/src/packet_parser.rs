
use std::str::Chars;
use std::iter::Peekable;

pub enum Command {
    Register { 
        id : String,
        ip : String,
        port : String,
    },
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

fn check_string( i1 : &str, i2 : &str ) -> Result<(), String> {
    if i1 == i2 {
        Ok(())
    }
    else {
        Err(format!("Expected {} but found {}", i1, i2))
    }
}

// [register|id:<symbol>|ip:<symbol>|port:<symbol>]
fn parse_register_request( parser : &mut Parser ) -> Result<Command, String> {
    parser.is( '|' )?;
    parser.next();
    let id_param_name = parser.symbol()?;
    check_string( &id_param_name[..], "id" )?;
    parser.is( ':' )?;
    parser.next();
    let id_param_value = parser.symbol()?;
    parser.is( '|' )?;
    parser.next();
    let ip_param_name = parser.symbol()?;
    check_string( &id_param_name[..], "ip" )?;
    parser.is( ':' )?;
    parser.next();
    let ip_param_value = parser.symbol()?;
    parser.is( '|' )?;
    parser.next();
    let port_param_name = parser.symbol()?;
    check_string( &port_param_name[..], "port" )?;
    parser.is( ':' )?;
    parser.next();
    let port_param_value = parser.symbol()?;
    parser.is( ']' )?;
    parser.next();

    Ok(Command::Register { id : id_param_value
                         , ip : ip_param_value
                         , port : port_param_value
                         })
}

pub fn parse( packet : &str ) -> Result<Command, String> {
    let mut parser = Parser{ orig : packet.chars().peekable(), done : false };

    parser.is( '[' )?;
    parser.next();
    let packet_type = parser.symbol()?;
    match &packet_type[..] { // TODO need to keep parsing to make sure there isn't more commands in the packet
        "register" => parse_register_request( &mut parser ),
        _ => Err("".to_string())
    }
}
