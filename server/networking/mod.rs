use super::database::database_actions::DbManager;
use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use super::database::{data_types::User, database_actions::QerryReturnType};
use anyhow::Result;

pub struct ServerNetworking {
    // channels: HashMap<u64, Vec<TcpStream>>,
    clients: Vec<TcpStream>,
}

impl ServerNetworking {
    pub const fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    pub fn handle_client(mut stream: TcpStream, _db_manager: Arc<DbManager>) -> Result<(), Error> {
        println!("Incoming connection from: {}", stream.peer_addr()?);
        let mut querries_vec: Vec<std::boxed::Box<tokio::task::JoinHandle<Result<QerryReturnType>>>>;//when a request is sent from the client, spawn a task, save it here and loop through this and return the data when a task finishes
        let mut buf = [0; 512];
        let mut _user: Option<User> = None; //I leave this here to remind you that as soon as the initial connection is made, packets containing the public keys should be sent.
                                            //This also implies user authentication and thus we can be sure which user is on this connection. For all future networking the
                                            //connection will bi encrypted so having the user (and his public key) in memory is beneficial

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
                        Self::handle_client(stream, temp)
                            .unwrap_or_else(|error| eprintln!("{error:?}"));
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
