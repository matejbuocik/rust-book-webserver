use hello_server::ThreadPool;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request = Request::build(&request_line).unwrap();

    let (status_code, filename) = match &request.path[..] {
        "/" => ("200 OK", "www/hello.html"),
        "/style.css" => ("200 OK", "www/style.css"),
        _ => ("404 Not Found", "www/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("HTTP/1.1 {status_code}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

struct Request {
    path: String,
    query: Option<String>,
}

impl Request {
    fn build(request_line: &str) -> Option<Request> {
        let mut path_query_iter = request_line.split(' ').nth(1).unwrap().split('?').take(2);

        let path = path_query_iter.next().unwrap().to_string();
        let query = path_query_iter.next().map(|q| q.to_string());

        Some(Request { path, query })
    }
}
