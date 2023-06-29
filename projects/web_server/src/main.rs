use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    // create a pool of 4 threads, will be able to process 4 requests concurrently

    for stream in listener.incoming() {
        // iterating through connection attempts

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // add buffering by managing calls to std::io::Read trait methods

    let http_request: Vec<_> = buf_reader
        .lines()
        // split the stream of data whenever there's a newline
        .map(|result| result.unwrap())
        // unwrap each result
        .take_while(|line| !line.is_empty())
        // http requests end with \n\n, so take each line until there is a line that is empty, created from the consecutive \n\n
        .collect();

    println!("Request: {:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // simulate slow request
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}