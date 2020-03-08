
use std::str::Chars;
use std::iter::Peekable;

pub enum Command {
    Register { 
        id : String,
        ip : String,
        port : String,
    },
    UdpTestReady,
    UdpTestSuccessful,
}

impl Command {
    pub fn to_packet(&self) -> &[u8]{
        match self {
            Command::UdpTestReady => "[udp_test_ready]\n".as_bytes(),
            Command::UdpTestSuccessful => "[udp_test_ready]\n".as_bytes(),
            _ => panic!("Unknown command encountered"),
        }
    }
}

// [<packet_type>|atom:<param>|list:<length>:<param1>:<param2>]

struct Parser<'a> {
    orig : Peekable<Chars<'a>>,
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
        self.orig.next();
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
        while c.is_alphanumeric() || c == '_' {
            self.orig.next();
            ret.push(c);
            c = *f(self.orig.peek())?; 
        }
        Ok(ret.into_iter().collect())
    }

    fn any(&mut self) -> Result<String, String> {
        fn f<T>( v : Option<T> ) -> Result<T, String> {
            match v {
                Some(v) => Ok(v),
                None => Err("Encountered end of stream in any".to_string()),
            }
        }

        let mut ret = vec![];
        let mut c = *f(self.orig.peek())?;
        while c != '|' && c != ']' {
            self.orig.next();
            ret.push(c);
            c = *f(self.orig.peek())?; 
        }
        Ok(ret.into_iter().collect())
    }

    fn done(&mut self) -> bool {
        let next = self.orig.peek();
        match next {
            None => true,
            _ => false,
        }
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
    check_string( &ip_param_name[..], "ip" )?;
    parser.is( ':' )?;
    parser.next();
    let ip_param_value = parser.any()?;
    parser.is( '|' )?;
    parser.next();
    let port_param_name = parser.symbol()?;
    check_string( &port_param_name[..], "port" )?;
    parser.is( ':' )?;
    parser.next();
    let port_param_value = parser.any()?;

    Ok(Command::Register { id : id_param_value
                         , ip : ip_param_value
                         , port : port_param_value
                         })
}

pub fn parse( packet : &str ) -> Result<Vec<Command>, String> {
    let mut parser = Parser{ orig : packet.chars().peekable() };

    let mut ret = vec![];
    while !parser.done() {
        parser.is( '[' )?;
        parser.next();
        let packet_type = parser.symbol()?;
        match &packet_type[..] { 
            "register" => ret.push(parse_register_request( &mut parser )?),
            "udp_test_ready" => ret.push( Command::UdpTestReady ),
            unknown => {return Err(format!("Encountered unknown comman name {}", unknown));},
        }
        parser.is( ']' )?;
        parser.next();
    }

    Ok(ret)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_parse_register() -> Result<(), String> {
        let commands = parse( "[register|id:some_id|ip:127.0.0.1|port:4000]" )?;
        assert_eq!( commands.len(), 1, "There should only be one command" );
        let register:Command = commands.into_iter().nth(0).unwrap();
        match register {
           Command::Register { id, ip, port } => {
               assert_eq!( id, "some_id", "id should be set correctly" );
               assert_eq!( ip, "127.0.0.1", "ip should be set correctly" );
               assert_eq!( port, "4000", "port should be set correctly" );
               Ok(())
           },
           _ => Err("Encountered non-register command".to_string()),
        }
    }

    #[test]
    fn should_parse_udp_test_ready() -> Result<(), String> {
        let commands = parse( "[udp_test_ready]" )?;
        assert_eq!( commands.len(), 1, "There should only be one command" );
        let register:Command = commands.into_iter().nth(0).unwrap();
        match register {
           Command::UdpTestReady => Ok(()),
           _ => Err("Encountered non-register command".to_string()),
        }
    }
}
