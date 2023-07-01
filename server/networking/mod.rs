use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;
use super::database::database_actions::DbManager;

pub struct ServerNetworking {
   // channels: HashMap<u64, Vec<TcpStream>>,
    clients : Vec<TcpStream>,
}


impl ServerNetworking{
    pub const fn new() -> Self {
        Self{
            clients : Vec::new(),
        }
    }

    pub fn handle_client(mut stream: TcpStream, db_manager: Arc<DbManager>) -> Result<(), Error> {
        println!("Incoming connection from: {}", stream.peer_addr()?);
        let mut buf = [0; 512];
        
        loop {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                return Ok(());
            }
            println!("Received message");
            stream.write_all(buf.get(..bytes_read).unwrap())?;
            println!("Echoed");
        }
    }

    pub fn listen_for_client(&mut self, db_manager: DbManager) {
        let db_manager = Arc::new(db_manager);
        //listen on port 8080
        let listener = TcpListener::bind("localhost:8080").unwrap();
        println!("Server listening on port 8080");

        // spawn a new thread for each connection
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {} ", stream.peer_addr().unwrap());
                    //self.clients.push(stream.try_clone().unwrap());
                    let temp = db_manager.clone();
                    thread::spawn(move || {
                        Self::handle_client(stream, temp).unwrap_or_else(|error| eprintln!("{error:?}"));
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

}