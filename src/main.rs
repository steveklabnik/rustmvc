extern crate nickel;
extern crate serialize;

use std::io::net::ip::Ipv4Addr;
use nickel::{Nickel, Request, Response, HttpRouter, StaticFilesHandler};

#[deriving(Decodable, Encodable)]
struct Todo {
    title: String,
    is_completed: bool,
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("frontend/"));

    server.get("/todos", todos_handler);
    
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}

fn todos_handler (request: &Request, response: &mut Response) { 
    response.send("{\"todos\":[]}"); 
}
