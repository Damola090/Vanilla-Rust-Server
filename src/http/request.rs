use super::method::{Method, MethodError};
use super::QueryString;
use crate::http::{method, request};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)] // Allow us to implement Debug trait

//All fields in Request are private
pub struct Request<'buf> {
    // buffer lifetime
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl <'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n..HEADERS...
    fn try_from<'a>(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            // Extract query string
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        // enumerate gives us a tuple (1, 2) - 1 is index, 2 is value
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

// // Extending the functionality of an already defined type be someone else--------
//     // external library or built in library

// impl TryFrom<&[u8]> for Request {
//     type Error = String;

//     fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
//         let string  = String::from("asd");
//         string.encrypt();
//         buf.encrypt();
//         unimplemented!()
//     }
// }
// trait Encrypt { // create an Interface called Encrypt
//     fn encrypt (&self) -> Self;
// }

// impl Encrypt for String { // Implement this Interface for datatype or String
//     fn encrypt (&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for &[u8] { // Implement this Interface for datatype or u8 Array
//     fn encrypt (&self) -> Self {
//         unimplemented!()
//     }
// }
