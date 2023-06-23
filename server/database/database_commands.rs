use anyhow::Result;
use tokio_postgres::Statement;

pub struct ServerCommands {
    pub save_message_statement: Statement,
    pub get_new_messages_statement: Statement,
}

impl ServerCommands {
    pub async fn new(client: &tokio_postgres::Client) -> Self {
        ServerCommands {
            save_message_statement: save_message(client).await,
            get_new_messages_statement: get_new_messages(client).await,
        }
    }
}

pub async fn get_new_messages(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::text::int4 AND message.id > $2::text::int4").await {
        Ok(statement) => {statement},
        Err(e) => {panic!("failed to prepare statement: {}", e)},
    }
}

pub async fn save_message(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("INSERT INTO messages (user_id, channel_id, text, date_created) VALUES ($1::text::int4, $2::text::int4, $3::text, $4::text::int8) RETURNING id").await {
        Ok(statement) => {statement},
        Err(e) => {panic!("failed to prepare statement: {}", e)},
    }
}
