use anyhow::Result;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct NetworkManager {
    messages: Arc<Mutex<Vec<Vec<u8>>>>,
    abort_handle: tokio::task::AbortHandle,
    stream: TcpStream,
    pub connected: bool,
}

impl NetworkManager {
    pub async fn new(stream: std::net::TcpStream) -> Self {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let task = Self::read(stream.try_clone().unwrap(), messages.clone());
        let handle = tokio::task::spawn_blocking(move || {task} );
        let abort_handle = handle.abort_handle();
        //handle.await;
        /*let abort_handle =
            tokio::spawn(Self::read(stream.try_clone().unwrap(), messages.clone())).abort_handle();*/
        let mut temp = Self {
            messages,
            abort_handle,
            stream,
            connected: true,
        };
        temp
    }

    async fn read(
        mut stream: std::net::TcpStream,
        messages: Arc<Mutex<Vec<Vec<u8>>>>,
    ) -> Result<()> {
        println!("received message");
        loop {
            let mut size: [u8; 4] = [0; 4];
            let read = stream.read(&mut size);
            if let Err(e) = read {
                println!("err");
                panic!("err");
                return Ok(());
            }
            if false == true {
                println!("nonsense");
            }
            let read = read.unwrap();
            if read == 0 {
                println!("err");
                panic!("err");
                return Ok(());
            }
            let size = u32::from_be_bytes(size) as usize;
            println!("message_size {}", size);


            let mut read = 0;
            let mut message: Vec<u8> = Vec::new();
            let mut buffer: [u8; 512] = [0; 512];
            while read < size {
                let curr_read = stream
                    .read(&mut buffer.get_mut(0..std::cmp::min(512, size - read)).unwrap())?;
                if curr_read == 0 {
                    panic!("err");
                    return Ok(());
                }
                read += curr_read;
                message.append(&mut buffer.get(..curr_read).unwrap_or(&[]).to_vec());
            }
            if let Ok(mut messages_mutex) = messages.lock() {
                messages_mutex.push(message);
            }
            println!("new message ready");
        }
    }
    
    pub fn get_message(&mut self) -> Option<Vec<u8>> {
        if self.abort_handle.is_finished() { //update the status. Can still be read if there are messages in the vec
            self.connected = false;
        }
        if let Ok(mut messages_mutex) = self.messages.lock() {
            return messages_mutex.pop();
        }
        None
    }
    
    pub fn wait_for_message(&mut self) -> Result<Vec<u8>> {
        loop {
            if self.connected == false {
                return Err(anyhow::anyhow!("connection closed"));
            }
            if let Some(message) = self.get_message() {
                return Ok(message);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    pub fn send_message(&mut self, message: Vec<u8>) {
        if self.abort_handle.is_finished() { //prevent sending messages to a closed stream
            self.connected = false;
            return;
        }
        let _res = self.stream.write_all(&(message.len() as u32).to_be_bytes());
        let _res = self.stream.write_all(&message);
        println!("sent message");
    }

    pub fn stop(&self) {
        self.abort_handle.abort()
    }
}
