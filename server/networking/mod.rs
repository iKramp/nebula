use crate::database::database_commands::save_message;

use super::database::database_actions::DbManager;
use super::database::{data_types::User, database_actions::QerryReturnType};
use alloc::sync::Arc;
use anyhow::Result;
use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::str;

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

    pub async fn handle_client(mut stream: TcpStream, _db_manager: Arc<DbManager>, client_id : u64) -> Result<()> {
        println!("Incoming connection from: {}", stream.peer_addr()?);
        let mut querries_vec: Vec<
            alloc::boxed::Box<tokio::task::JoinHandle<Result<QerryReturnType>>>,
        > = Vec::new(); //when a request is sent from the client, spawn a task, save it here and loop through this and return the data when a task finishes
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
            //stream.write_all(buf.get(..bytes_read).ok_or(anyhow::anyhow!("err"))?)?;
            //println!("Echoed");
        
            //decide what to do depending on the client request
            // 1 - client requests its id
            // 2 - client sends a message 
            // 3 - client wants new messages ig
            if buf[0] == 1 {
                println!("returning id");
                stream.write_all(&client_id.to_be_bytes());
            }
            else if buf[0] == 2{
                println!("saving message");
                // needs async?
                let msg = crate::database::data_types::Message { 
                    id: 1,
                    user_id: client_id, 
                    channel_id: 1, 
                    text: str::from_utf8(&buf[1..bytes_read]).unwrap().to_string(), 
                    date_created: 1 
                };
                let tman = _db_manager.clone();
                let handle = tokio::spawn(async move {
                    tman.save_message(&msg).await
                });
                querries_vec.push(std::boxed::Box::new(handle));
            }
            else if buf[0] == 3{
                

            }
            for (id, handle) in querries_vec.iter_mut().enumerate() {
                if handle.is_finished() {
                    let (res,) = tokio::join!(handle);//use res to return a value
                    querries_vec.remove(id);
                    break;//we break so we have no borrow conflicts. returning 1 result per loop is sufficient anyway
                }
            }
        }
    }
    
    pub async fn listen_for_client(&mut self, db_manager: DbManager) {
        let db_manager = Arc::new(db_manager);
        //listen on port 8080
        let listener = TcpListener::bind("localhost:8080").unwrap();
        println!("Server listening on port 8080");

        let mut client_cnt = 0;
        // spawn a new thread for each connection
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    client_cnt += 1;
                    println!("New connection: {} ", stream.peer_addr().unwrap());
                    //self.clients.push(stream.try_clone().unwrap());
                    let temp = db_manager.clone();
                    let handle = tokio::spawn(async move {
                        Self::handle_client(stream, temp, client_cnt).await
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
