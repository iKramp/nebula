use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

pub fn handle_client(mut stream : TcpStream){
    let mut data = [0 as u8; 100]; // data buffer
    while match stream.read(&mut data){
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub async fn listen_for_client() {
    //listen on port 8080
    let listener = TcpListener::bind("localhost:8080").unwrap();
    println!("Server listening on port 8080");

    // spawn a new thread for each connection
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection: {} ",stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) =>{
                println!("Error: {}",e);
            }
        }
    }
    //close socket server
    println!("Stopping listening");
    drop(listener);
}