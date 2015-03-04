#![feature(net)]
#![feature(io)]

extern crate hyper;

use std::io::Write;
use std::net::IpAddr;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn hello(_: Request, res: Response<Fresh>) {
    let mut res = res.start().unwrap();
    res.write_all(b"Hello World!").unwrap();
    res.end().unwrap();
}

fn main() {
    Server::http(hello).listen(IpAddr::new_v4(127, 0, 0, 1), 3000).unwrap();
}
