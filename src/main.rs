extern crate nickel;
extern crate postgres;
extern crate serialize;

use std::io::net::ip::Ipv4Addr;
use nickel::{Nickel, Request, Response, HttpRouter, StaticFilesHandler};

use postgres::{PostgresConnection, NoSsl};

use serialize::json;

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
    let conn = PostgresConnection::connect("postgres://rustmvc@localhost",
                                           &NoSsl).unwrap();

    let stmt = conn.prepare("SELECT title, is_completed FROM todos")
        .unwrap();
    let results = stmt.query([]).unwrap().map(|row| {
        Todo {
            title: row.get(0u),
            is_completed: row.get(1u),
        }
    }).collect::<Vec<Todo>>();

    let results = json::encode(&results);

    response
        .content_type("json")
        .send(format!("{{\"todos\":{}}}", results)); 
}
