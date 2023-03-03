use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Here we set up a listener to read from the stream, the tcp listener looks for connections on a specific port
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener = match listener {
        Ok(v) => v,
        Err(e) => panic!("Listener not found corerctly: {}", e)
    };

    // Listen to incoming streams and handle the connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    } 
}

/// Function to handle incoming streams - reading the bytes directly
fn handle_connection (mut stream: TcpStream){
    // Create buffer to hold data 
    let mut buffer = [0; 1024];

    // read our stream to the buffger
    stream.read(&mut buffer).unwrap();

    // Define our routes
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Now we check that the start of the buffer refers to on of our routes
    let (status_code, filename) = {
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        }
        else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")

        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    // Get the file contents
    let contents = fs::read_to_string(format!("static/{}", filename)).unwrap();

    // Response - HTTP-version Status-code Reason-phrase 
    // headers 
    // message-body
    let response = format!(
        "{}\r\n Content-Length: {}\r\n\r\n{}",
        status_code, 
        contents.len(),
        contents
    );
    
    // write the response and flush
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}