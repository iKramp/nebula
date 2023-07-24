use anyhow::Result;
use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct NetworkManager {
    messages: Arc<Mutex<Vec<Vec<u8>>>>,
    abort_handle: tokio::task::AbortHandle,
    stream: TcpStream,
}

impl NetworkManager {
    pub async fn new(stream: std::net::TcpStream) -> Self {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let abort_handle =
            tokio::spawn(Self::read(stream.try_clone().unwrap(), messages.clone())).abort_handle();
        let mut temp = Self {
            messages,
            abort_handle,
            stream,
        };
        temp
    }

    async fn read(
        mut stream: std::net::TcpStream,
        messages: Arc<Mutex<Vec<Vec<u8>>>>,
    ) -> Result<()> {
        loop {
            let mut size: [u8; 4] = [0; 4];
            stream.read(&mut size)?;
            let size = u32::from_be_bytes(size) as usize;

            let mut read = 0;
            let mut message: Vec<u8> = Vec::new();
            let mut buffer: [u8; 512] = [0; 512];
            while read < size {
                let curr_read = stream
                    .read(&mut buffer.get_mut(0..std::cmp::min(512, size - read)).unwrap())?;
                message.append(&mut buffer.get(..curr_read).unwrap_or(&[]).to_vec());
            }
            if let Ok(mut messages_mutex) = messages.lock() {
                messages_mutex.push(message);
            }
        }
    }
    
    fn get_message(&mut self) -> Option<Vec<u8>> {
        if let Ok(mut messages_mutex) = self.messages.lock() {
            return messages_mutex.pop();
        }
        None
    }
    
    fn send_message(&mut self, message: Vec<u8>) {
        let _res = self.stream.write_all(&message.len().to_be_bytes());
        let _res = self.stream.write_all(&message);
    }

    pub fn stop(&self) {
        self.abort_handle.abort()
    }
}
