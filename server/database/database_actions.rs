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
        &mut self,
        message: data_types::Message,
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
        &mut self,
        channel_id: u64,
        last_message_id: u64,
        client: &tokio_postgres::Client,
    ) -> Result<Vec<data_types::Message>> {
        let res = client.query(
            &self.commands.get_new_messages_statement,
            &[
                &channel_id.to_string(),
                &last_message_id.to_string(),
            ]).await?;
        
        //TODO parse
        
        Ok(vec![])
    }
}
