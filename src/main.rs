extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use nickel::{Nickel, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("frontend/"));
    
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
