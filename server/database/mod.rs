mod data_types;
pub mod database_actions;
pub mod database_commands;
mod tests;

use anyhow::Result;
use std::env;
use tokio_postgres::NoTls;

pub async fn connect_to_db(db_name: &str) -> Result<tokio_postgres::Client> {
    let args: Vec<String> = env::args().collect();
    let username;
    let password;
    let host;
    if let Some(arg) = args.get(1) {
        username = arg;
    } else {
        anyhow::bail!("postgres username argument was not specified")
    }

    if let Some(arg) = args.get(2) {
        password = arg;
    } else {
        anyhow::bail!("postgres password argument was not specified")
    }

    if let Some(arg) = args.get(3) {
        host = arg;
    } else {
        anyhow::bail!("postgres host argument was not specified")
    }

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        format!("host={host} user={username} password={password} dbname = {db_name}").as_str(),
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    Ok(client)
}