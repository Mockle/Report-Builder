#![allow(non_snake_case)]

extern crate reportbuilder;
use reportbuilder::threadpool::ThreadPool;
use reportbuilder::server::Connection;

use std::net::TcpListener;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();        

        pool.execute(|| {
            let con = Connection::new(stream);
            Connection::respond(con);
        })
    }

    println!("Shutting down.");
}