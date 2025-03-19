use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();


        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let request_line = BufReader::new(&mut stream).lines().next().unwrap().unwrap();
    
    let (status_line, filename) = if request_line.contains("GET /bad HTTP/1.1") {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    } else {
        ("HTTP/1.1 200 OK", "hello.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{contents}", contents.len());

    stream.write_all(response.as_bytes()).unwrap();
}