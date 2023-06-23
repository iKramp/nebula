#![allow(unused_imports)]
use anyhow::{Ok, Result};
use std::env;
use tokio_postgres::NoTls;
mod database;
use database::database_commands;

async fn connect_to_db(username: &str) -> Result<tokio_postgres::Client> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        format!("host=localhost user={} dbname = mydb", username).as_str(),
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let username;
    if let Some(arg) = args.get(1) {
        username = arg;
    } else {
        anyhow::bail!("postgres username argument was not specified")
    }

    let client = connect_to_db(username).await?;
    
    let _db_manager = database::database_actions::DbManager::new(&client).await;

    println!("finished executing");
    Ok(())
}
