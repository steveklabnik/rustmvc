extern crate postgres;

use postgres::{Connection, NoSsl};

fn main() {
    let conn = Connection::connect("postgres://rustmvc@localhost",
                                           &NoSsl).unwrap();

    conn.execute("CREATE TABLE todos (
                    id              SERIAL PRIMARY KEY,
                    title           VARCHAR NOT NULL,
                    is_completed    BOOLEAN NOT NULL
                  )", []).unwrap();
}

