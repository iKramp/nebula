use super::init_db;
use tokio_postgres::{Client, NoTls};
use std::env;
use futures::executor::block_on;
use anyhow::{Result, Error};

const TEST_DB: &str = "testdb";

fn clear_db() {
    if let Err(e) = super::init_db::drop_db(&TEST_DB) {
        panic!("{}", e);
    }
    if let Err(e) = super::init_db::create_db(&TEST_DB) {
        panic!("{}", e);
    }
}

async fn get_client() -> tokio_postgres::Client {
    let args = format!("host=localhost user=postgres dbname = {}", TEST_DB);
    let f = tokio_postgres::connect(
        &args,
        NoTls
    ).await;
    let (client, connection) = f.expect("couldn't connect");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_save_message() {
        super::clear_db();


        let date_created: u64 = 1635676478;
        let user_id = 1;
        let channel_id = 1;
        let text = "test message".to_string();

        let command = crate::database::server_commands::save_message(user_id, channel_id, text.clone(), date_created);

        let client = super::get_client().await;

        let res = client.execute(&*command.command, &[]).await;

        if let Err(e) = res {
            panic!("{}", e)
        }
    }
}