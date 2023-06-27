use std::io::{Read, Write,Error};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

pub fn handle_client(mut stream: TcpStream) -> Result<(),Error> {
    println!("Incoming connection from: {}",stream.peer_addr()?);
    let mut buf = [0;512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(());}
        println!("Received message");
        stream.write_all(&buf[..bytes_read])?;
    }
}

pub fn listen_for_client() {
    //listen on port 8080
    let listener = TcpListener::bind("localhost:8080").unwrap();
    println!("Server listening on port 8080");

    // spawn a new thread for each connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {} ", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
                });
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
    //close socket server
    println!("Stopping listening");
    drop(listener);
}
