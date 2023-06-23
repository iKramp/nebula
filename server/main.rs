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

    let _database_commands = database::database_commands::DatabaseCommands::new(&client).await;

    //this is basically a test of some sort but please don't delete it, i don't wanna go through the hell of figuring out how to retrieve the columns again.
    //because the columns can have any type, retrieving them is very unintuitive
    /*let get_new_message_command = client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::text::int4 AND message.id > $2::text::int4").await?;
    let save_message_command = client.prepare("INSERT INTO messages (user_id, channel_id, text, date_created) VALUES ($1::text::int4, $2::text::int4, $3::text, $4::text::int8) RETURNING id").await?;

    client
        .query(
            &save_message_command,
            &[&"1", &"1", &"test message", &"345263546"],
        )
        .await?;
    let res = client
        .query(&get_new_message_command, &[&"1", &"4"])
        .await?;

    for row in res {
        for column in row.columns().iter().enumerate() {
            if column.1.type_().to_string() == "text" {
                let val: String = row.get(column.0);
                println!("{}", val);
            }
        }
    }*/

    println!("finished executing");
    Ok(())
}
