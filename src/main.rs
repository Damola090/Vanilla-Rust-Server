#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));

    // let _get = Method::GET("abcd".to_string());
    // let _delete = Method::DELETE(100);
    // let _post = Method::POST;
    // let _put = Method::PUT;
    // let _head = Method::HEAD;

    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..14];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";

    // dbg!(&string);
    // dbg!(string_slice);
}
