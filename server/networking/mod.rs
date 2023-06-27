use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

pub fn handle_client(mut stream: TcpStream) {
    let mut data = [0_u8; 100]; // data buffer
    while if let Ok(size) = stream.read(&mut data) {
    stream.write_all(data.get(0..size).unwrap()).unwrap();
    true
} else {
    println!(
        "An error occurred, terminating connection with {}",
        stream.peer_addr().unwrap()
    );
    stream.shutdown(Shutdown::Both).unwrap();
    false
    } {}
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
                    handle_client(stream);
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
