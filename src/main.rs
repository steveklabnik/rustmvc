extern crate nickel;
extern crate postgres;
extern crate serialize;

use std::io::net::ip::Ipv4Addr;
use nickel::{Nickel, Request, Response, HttpRouter, StaticFilesHandler};

use postgres::{PostgresConnection, NoSsl};

use std::collections::TreeMap;
use serialize::json::ToJson;
use serialize::json;

#[deriving(Decodable, Encodable)]
struct Todo {
    id: i32,
    title: String,
    is_completed: bool,
}

// Specify encoding method manually
impl ToJson for Vec<Todo> {
    fn to_json(&self) -> json::Json {
        let todos = self.iter().map({|todo| json::encode(todo) }).collect::<Vec<_>>();

        let mut d = TreeMap::new();
        d.insert("todos".to_string(), todos.to_json());
        
        json::Object(d)
    }
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

    let stmt = conn.prepare("SELECT id, title, is_completed FROM todos")
        .unwrap();
    let results = stmt.query([]).unwrap().map(|row| {
        Todo {
            id: row.get(0u),
            title: row.get(1u),
            is_completed: row.get(2u),
        }
    }).collect::<Vec<Todo>>();

    let results = json::encode(&results);

    response
        .content_type("json")
        .send(format!("{{\"todos\":{}}}", results)); 
}
