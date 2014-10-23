extern crate nickel;
extern crate postgres;
extern crate serialize;
extern crate http;

use http::status;
use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter, StaticFilesHandler, JsonBody, QueryString };

use postgres::{ PostgresConnection, NoSsl };

use std::collections::TreeMap;
use serialize::json::ToJson;
use serialize::json;

#[deriving(Decodable, Encodable)]
struct Todo {
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
    server.utilize(Nickel::json_body_parser());
    server.utilize(Nickel::query_string());

    server.get("/todos", get_todos);
    server.post("/todos", post_todo);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}

fn get_todos (request: &Request, response: &mut Response) {
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

fn post_todo (request: &Request, response: &mut Response) {
    match request.json_as::<Todo>() {
        Some(t) => response.send(format!("{}", json::encode(&t))),
        None => response.status_code(http::status::BadRequest).send("{\"error\":\"cannot be parsed\"}"),
    };
}

