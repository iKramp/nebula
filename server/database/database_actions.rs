use super::data_types;
use super::database_commands;
use anyhow::{Ok, Result};

pub struct DbManager {
    commands: database_commands::DatabaseCommands,
}

impl DbManager {
    pub async fn new(client: &tokio_postgres::Client) -> Self {
        DbManager {
            commands: database_commands::DatabaseCommands::new(client).await,
        }
    }

    pub async fn save_message(
        &self,
        message: &data_types::Message,
        client: &tokio_postgres::Client,
    ) -> Result<()> {
        client
            .execute(
                &self.commands.save_message_statement,
                &[
                    &message.user_id.to_string(),
                    &message.channel_id.to_string(),
                    &message.text,
                    &message.date_created.to_string(),
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_new_messages(
        &self,
        channel_id: u64,
        last_message_id: u64,
        client: &tokio_postgres::Client,
    ) -> Result<Vec<data_types::Message>> {
        let rows = client.query(
            &self.commands.get_new_messages_statement,
            &[
                &channel_id.to_string(),
                &last_message_id.to_string(),
            ]).await?;
        
        let mut message_vec = Vec::new();

        //this can panic, but it should if anything is wrong so we know immediately
        for row in rows {
            let id: i64 = row.get(0);
            let user_id: i64 = row.get(1);
            let channel_id: i64 = row.get(2);
            let text: String = row.get(3);
            let date_created: i64 = row.get(4);
            message_vec.push(data_types::Message::new(
                id as u64,
                user_id as u64,
                channel_id as u64,
                &text,
                date_created as u64,
            ))
        }

        Ok(message_vec)
    }
}
