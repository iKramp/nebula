#![allow(clippy::panic)] //something's wrong if it can't prepare the statements and the server can't continue anyway

use anyhow::Result;
use tokio_postgres::Statement;

pub struct DatabaseCommands {
    //TODO i think this will get too many arguments, maybe redo?
    pub save_message_statement: Statement,
    pub get_new_messages_statement: Statement,
    pub get_last_n_messages_statement: Statement,
    pub get_n_messages_before_statement: Statement,
    pub add_user_statement: Statement,
    pub add_channel_statement: Statement,
    pub add_user_channel_link: Statement,
    pub get_user_channels: Statement,
}

impl DatabaseCommands {
    pub async fn new(client: &tokio_postgres::Client) -> Self {
        Self {
            save_message_statement: save_message(client).await,
            get_new_messages_statement: get_new_messages(client).await,
            get_last_n_messages_statement: get_last_n_messages(client).await,
            get_n_messages_before_statement: get_n_messages_before(client).await,
            add_user_statement: add_user(client).await,
            add_channel_statement: add_channel(client).await,
            add_user_channel_link: add_user_channel_link(client).await,
            get_user_channels: get_user_channels(client).await,
        }
    }
}

pub async fn get_new_messages(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::int8 AND message.id > $2::int8").await {
        Ok(statement) => {statement},
        Err(e) => {panic!("failed to prepare statement: {}", e)},
    }
}

pub async fn save_message(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("INSERT INTO messages (user_id, channel_id, text, date_created) VALUES ($1::text::int8, $2::text::int8, $3::text, $4::text::int8) RETURNING id").await {
        Ok(statement) => { statement },
        Err(e) => {panic!("failed to prepare statement: {}", e)},
    }
}

pub async fn get_last_n_messages(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::int8 ORDER BY message.date_created DESC LIMIT $2::int8").await {
        Ok(statement) => { statement },
        Err(e) => { panic!("failed to prepare statement: {}", e)}
    }
}

pub async fn get_n_messages_before(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::int8 AND message.id < $2::int8 ORDER BY message.date_created DESC LIMIT $3::int8").await {
        Ok(statement) => { statement },
        Err(e) => { panic!("failed to prepare statement: {}", e) }
    }
}

pub async fn add_user(client: &tokio_postgres::Client) -> Statement {
    match client
        .prepare("INSERT INTO users (name) VALUES ($1::text) RETURNING id")
        .await
    {
        Ok(statement) => statement,
        Err(e) => {
            panic!("failed to prepare statement: {}", e)
        }
    }
}

pub async fn add_channel(client: &tokio_postgres::Client) -> Statement {
    match client
        .prepare("INSERT INTO channels (name) VALUES ($1::text) RETURNING id")
        .await
    {
        Ok(statement) => statement,
        Err(e) => {
            panic!("failed to prepare statement: {}", e)
        }
    }
}

pub async fn add_user_channel_link(client: &tokio_postgres::Client) -> Statement {
    match client
        .prepare("INSERT INTO channel_user_links (channel_id, user_id) VALUES ($1::int8, $2::int8) RETURNING id")
        .await
    {
        Ok(statement) => statement,
        Err(e) => {
            panic!("failed to prepare statement: {}", e)
        }
    }
}

pub async fn get_user_channels(client: &tokio_postgres::Client) -> Statement {
    match client
        .prepare("SELECT * FROM channels AS channel WHERE EXISTS (SELECT channel_id FROM channel_user_links WHERE user_id = $1::int8 AND channel.id = channel_id)")
        .await
    {
        Ok(statement) => statement,
        Err(e) => {
            panic!("failed to prepare statement: {}", e)
        }
    }
}
