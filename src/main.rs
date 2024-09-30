use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    iter::once,
    net::TcpListener,
    path::Path,
};

use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};

fn main() -> () {
    listen_to_streams()
}

fn listen_to_streams() {
    let listener = create_tcp_listener("127.0.0.1:8080");
    println!("Listening on: {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}

fn create_tcp_listener(host: &str) -> TcpListener {
    // TODO: is there a better way to handle this error?
    let listener = TcpListener::bind(host)
        .unwrap_or_else(|error| panic!("Failed to bind to address: {}, error = {}", host, error));
    listener
}

fn handle_client(mut stream: std::net::TcpStream) {
    println!("Connection established!");

    let http_request = buffer_stream_and_parse_request(&mut stream);
    println!("Request: {:?}", http_request);

    let path = get_path_from_request(http_request);
    println!("Request path: {}", path);

    let html_response = read_static_file(&path);
    let mimetype = get_mimetype_from_path(&path);
    println!("Mimetype: {}", mimetype);

    let response = create_http_response(mimetype, &html_response);
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&html_response).unwrap();
    println!("Response sent! \n");
}

fn buffer_stream_and_parse_request(stream: &mut std::net::TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    return buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
}

fn get_path_from_request(http_request: Vec<String>) -> String {
    let request_path = http_request[0].split_whitespace().nth(1).unwrap();
    if request_path == "/" {
        "index.html".to_string()
    } else {
        request_path.to_string()
    }
}

fn read_static_file(path: &String) -> Vec<u8> {
    let parent_path = "public";
    let file_path = format!("{}/{}", parent_path, path);
    return read_file(&file_path);
}

fn get_mimetype_from_path(path: &String) -> &'static str {
    let extension = path.split('.').last().unwrap();
    match extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        _ => "text/plain",
    }
}

fn read_file(path: &String) -> Vec<u8> {
    let mut file_content = Vec::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}

fn create_http_response(content_type: &str, body: &Vec<u8>) -> String {
    format!(
        "HTTP/1.1 200 OK \r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type,
        body.len()
    )
}
