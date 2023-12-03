#![allow(unused_variables, unused_imports)]
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    let path = args.get(1);

    let file = File::open("index.html")?;

    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8111";

    let end_point: String = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(&end_point).unwrap();

    println!("Server listening on {}", end_point);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)?;

        println!("Connection established!");
    }

    Ok(())
}

fn handle_index_html() -> Result<(), std::io::Error> {
    let file = File::open("index.html")?;

    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|r| !r.is_empty())
        .collect();

    println!("{:?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}
