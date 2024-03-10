// Every File in rust is treated as a Module
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request (&mut self, request: &Request) -> Response;
    fn  handle_bad_request (& mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }

}


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        // Self is an Alias for Server
        Self { addr: (addr) }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server is currently listening on Port {}", self.addr);

        // In the result is ok - return tcp listener,
        // if result is err - terminate conn - loginfo
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                //code will not compile until we cover all possble outcome
                Ok((mut stream, _address)) => {
                    let mut buffer = [0; 1024]; //allocate 1024 bytes List to read incoming request
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response= match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    handler.handle_request(&request)
                                    
                                }
                                Err(e) => {
                                    println!("Failed to Parse a Request {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send Response: {}", e);
                            }

                            println!("Rececive a Request {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => {
                            println!("Failed to Read from connection {}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establlish the connection {}", e)
                }
            }
        }
    }
}
