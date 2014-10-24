extern crate nickel;
extern crate postgres;
extern crate serialize;
extern crate http;

use http::status;
use std::io::net::ip::Ipv4Addr;
use nickel::{
    Nickel,
    Request,
    Response,
    HttpRouter,
    StaticFilesHandler,
    JsonBody,
    QueryString
};

use postgres::{
    PostgresConnection,
    NoSsl
};

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
    server.utilize(Nickel::json_body_parser());
    server.utilize(Nickel::query_string());

    server.get( "/todos", get_todos);
    server.post("/todos", post_todo);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}

fn get_todos(request: &Request, response: &mut Response) {
    // this isn't super secure but it's also just a toy so whatever
    let conn = PostgresConnection::connect("postgres://rustmvc@localhost",
                                           &NoSsl).unwrap();

    let stmt = conn.prepare("SELECT id, title, is_completed FROM todos").unwrap();
    let results = stmt.query([]).unwrap().map(|row| {
        Todo {
            id: row.get(0u),
            title: row.get(1u),
            is_completed: row.get(2u),
        }
    }).collect::<Vec<Todo>>();

    let results = results.to_json();

    response
        .content_type("json")
        .send(format!("{}", results));
}

fn post_todo(request: &Request, response: &mut Response) {
    let (status, body) = match request.json_as::<Todo>() {
        Some(t) => (http::status::Created, store_todo(t)),
        None => (http::status::BadRequest, "{\"error\":\"cannot be parsed\"}".to_string()),
    };

    response.status_code(status).send(body);
}

fn store_todo(todo: Todo) -> String {
    let conn = PostgresConnection::connect("postgres://rustmvc@localhost", &NoSsl).unwrap();

    conn.execute("INSERT INTO todos (title, is_completed) VALUES ($1, $2)",
                 &[&todo.title, &todo.is_completed]).unwrap();

    json::encode(&todo)
}
