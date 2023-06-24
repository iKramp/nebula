use anyhow::Result;
use tokio_postgres::Statement;

pub struct DatabaseCommands {
    pub save_message_statement: Statement,
    pub get_new_messages_statement: Statement,
    pub get_last_n_messages_statement: Statement,
}

impl DatabaseCommands {
    pub async fn new(client: &tokio_postgres::Client) -> Self {
        DatabaseCommands {
            save_message_statement: save_message(client).await,
            get_new_messages_statement: get_new_messages(client).await,
            get_last_n_messages_statement: get_last_n_messages(client).await,
        }
    }
}

pub async fn get_new_messages(client: &tokio_postgres::Client) -> Statement {
    match client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::text::int8 AND message.id > $2::text::int8").await {
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
    match client.prepare("SELECT * FROM messages AS message WHERE message.channel_id = $1::text::int8 ORDER BY message.date_created DESC LIMIT $2::text::int8").await {
        Ok(statement) => { statement },
        Err(e) => { panic!("failed to prepare statement: {}", e)}
    }
}
