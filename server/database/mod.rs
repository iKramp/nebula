mod data_types;
pub mod database_actions;
pub mod database_commands;
mod tests;

use anyhow::Result;
use std::env;
use tokio_postgres::NoTls;

pub async fn connect_to_db() -> Result<tokio_postgres::Client> {
    let args: Vec<String> = env::args().collect();
    let username;
    if let Some(arg) = args.get(1) {
        username = arg;
    } else {
        anyhow::bail!("postgres username argument was not specified")
    }

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        format!("host=localhost user={username} dbname = mydb").as_str(),
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