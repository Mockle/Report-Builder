use std::io::prelude::*;
use std::net::TcpStream;

use std::fs::File;
use std::collections::HashMap;

pub struct Connection{
    response: String,
    stream: TcpStream
}

impl Connection{
    pub fn new(mut stream: TcpStream) -> Connection{
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let route = match parse_buffer(&buffer){
            Ok(route) => route,
            Err(err) => panic!("Err {:?}", err)
        };

        Connection{
            response: construct_reponse(&route),
            stream: stream
        }
    }

    pub fn respond(mut con: Connection){
        con.stream.write(con.response.as_bytes()).unwrap();
        con.stream.flush().unwrap(); 
    }
}

fn parse_buffer(buffer: &[u8; 512]) -> (Result<String, String>){
    let buffer_string = String::from_utf8_lossy(buffer);
    let request_vec: Vec<&str> = buffer_string.split(" ").take(2).collect();

    if request_vec[0] != "GET"{
        return Err(String::from("Request was of invalid type!"))
    }

    return Ok(request_vec[1].to_owned());
}

fn construct_reponse(route: &str) -> (String){
    let (status_line, file_name) = path_from_route(route);

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return format!("{}{}", status_line, contents);
}

fn path_from_route(route: &str) -> (&str, &str){
    println!("Requested route: {}", route);

    let mut routes = HashMap::new();
    routes.insert("/", "site/index.html");
    routes.insert("/sub/sub", "site/sub/sub.html");

    if let Some(value) = routes.get(&route){
        return ("HTTP/1.1 200 OK\r\n\r\n", value);
    }else{
        return ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "site/404.html");
    }
}