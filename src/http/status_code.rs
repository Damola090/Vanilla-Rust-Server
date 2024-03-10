use std::fmt::{ Display, Formatter, Result as FmtResult };

#[derive(Copy, Clone, Debug)] 
// Drive Copy and Clone Trait for the status Code
// Good Pratice - Derive Debug Trait for all of types so that later u can log the easily

pub enum StatusCode { // enum Attributes
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str { // enum Methods
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    } 
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}




