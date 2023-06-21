#![allow(unused_imports)]
use anyhow::{Ok, Result};
use std::env;
use tokio_postgres::NoTls;
mod database;
use database::server_commands;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let username;
    if let Some(arg) = args.get(1) {
        username = arg;
    } else {
        anyhow::bail!("postgres username argument was not specified")
    }

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

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    let command_1 = server_commands::save_message(1, 1, "tewuibewoifhnwqe".to_owned(), 15243562546);
    let command_2 = server_commands::get_new_messages(1, 3);
    println!("{}", command_1.command);
    println!("{}", command_2.command);


    Ok(())
}
