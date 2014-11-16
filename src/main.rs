extern crate nickel;
extern crate postgres;
extern crate serialize;
extern crate http;
extern crate r2d2;
extern crate r2d2_postgres;

use http::status;
use std::io::net::ip::Ipv4Addr;
use nickel::{
    Nickel,
    Request,
    Response,
    HttpRouter,
    StaticFilesHandler,
    JsonBody,
    Middleware,
    MiddlewareResult,
    Continue,
};

use postgres::{
    Connection,
    NoSsl
};

use r2d2_postgres::PostgresPoolManager;
use r2d2::PoolManager;

use std::collections::TreeMap;
use serialize::json::{ToJson, Json};
use serialize::json;

#[deriving(Decodable, Encodable)]
struct Todo {
    id: i32,
    title: String,
    is_completed: bool,
}

impl ToJson for Todo {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("id".to_string(), self.id.to_json());
        d.insert("title".to_string(), self.title.to_json());
        d.insert("is_completed".to_string(), self.is_completed.to_json());
        json::Object(d)
    }
}

struct ConnectionPool {
    pool: PostgresPoolManager,
}

impl ConnectionPool {
    fn new() -> ConnectionPool {
        // this isn't super secure but it's also just a toy so whatever
        ConnectionPool {
            pool: PostgresPoolManager::new("postgres://rustmvc@localhost", NoSsl),
        }
    }
}

impl Middleware for ConnectionPool {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        println!("Connection pool middleware called");
        let conn = self.pool.connect().ok().expect("could not grab a connection");

        req.map.insert(conn);

        Ok(Continue)
    }
}

fn main() {
    let mut server = Nickel::new();
    let port = 6767u16;


    server.utilize(StaticFilesHandler::new("frontend/"));
    server.utilize(Nickel::json_body_parser());
    server.utilize(Nickel::query_string());
    server.utilize(ConnectionPool::new());

    server.get( "/todos", get_todos);
    server.post("/todos", post_todo);

    println!("Server listening on port {}", port);
    server.listen(Ipv4Addr(127, 0, 0, 1), port);
}

fn get_todos(req: &Request, _: &mut Response) -> Json {
    let opt_conn: Option<&Connection> = req.map.get();
    let conn = opt_conn.unwrap();

    let stmt = conn.prepare("SELECT id, title, is_completed FROM todos").unwrap();
    let results = stmt.query([]).unwrap().map(|row| {
        Todo {
            id: row.get(0u),
            title: row.get(1u),
            is_completed: row.get(2u),
        }
    }).collect::<Vec<Todo>>();

    let mut d = TreeMap::new();
    d.insert("todos".to_string(), results.to_json());

    d.to_json()
}

fn post_todo(req: &Request, _: &mut Response) -> (status::Status, String) {
    println!("called post_todo");
    let opt_conn: Option<&Connection> = req.map.get();
    let conn = opt_conn.unwrap();

    match req.json_as::<Todo>() {
        Some(t) => (http::status::Created, store_todo(t, conn)),
        None => (http::status::BadRequest, "{\"error\":\"cannot be parsed\"}".to_string()),
    }
}

fn store_todo(todo: Todo, conn: &Connection) -> String {
    println!("called store_todo");

    conn.execute("INSERT INTO todos (title, is_completed) VALUES ($1, $2)",
                 &[&todo.title, &todo.is_completed]).unwrap();

    json::encode(&todo)
}
