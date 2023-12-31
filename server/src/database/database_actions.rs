use super::data_types;
use super::database_commands;
use crate::database::data_types::DbType;
use anyhow::{Ok, Result};

#[allow(unused)]
pub enum QerryReturnType {
    None,
    U64(u64),
    Messages(Vec<data_types::Message>),
    Channels(Vec<data_types::Channel>),
    Users(Vec<data_types::User>),
}

pub struct DbManager {
    commands: database_commands::DatabaseCommands,
    client: tokio_postgres::Client,
}

impl DbManager {
    pub async fn new(client: tokio_postgres::Client) -> Self {
        Self {
            commands: database_commands::DatabaseCommands::new(&client).await,
            client,
        }
    }

    #[allow(unused)]
    pub async fn save_message(&self, message: &data_types::Message) -> Result<QerryReturnType> {
        let res = self
            .client
            .query(
                &self.commands.save_message_statement,
                &[
                    &message.user_id.to_string(),
                    &message.channel_id.to_string(),
                    &message.text,
                    &message.date_created.to_string(),
                ],
            )
            .await?;
        let row = res
            .get(0)
            .ok_or_else(|| anyhow::format_err!("Database error at adding message"))?;
        Ok(QerryReturnType::U64(row.try_get::<_, i64>(0)? as u64))
    }

    #[allow(unused)]
    pub async fn get_new_messages(
        &self,
        channel_id: u64,
        last_message_id: u64,
    ) -> Result<QerryReturnType> {
        let rows = self
            .client
            .query(
                &self.commands.get_new_messages_statement,
                &[&(channel_id as i64), &(last_message_id as i64)],
            )
            .await?;
        Ok(QerryReturnType::Messages(
            data_types::Message::from_db_rows(rows)?,
        ))
    }

    #[allow(unused)]
    pub async fn get_last_n_messages(
        &self,
        channel_id: u64,
        number_of_messages: u8, //ye let's not allow big numbers
    ) -> Result<QerryReturnType> {
        let rows = self
            .client
            .query(
                &self.commands.get_last_n_messages_statement,
                &[&(channel_id as i64), &(number_of_messages as i64)],
            )
            .await?;
        Ok(QerryReturnType::Messages(
            data_types::Message::from_db_rows(rows)?,
        ))
    }

    #[allow(unused)]
    pub async fn get_n_messages_before(
        &self,
        channel_id: u64,
        before_message_id: u64,
        number_of_messages: u8,
    ) -> Result<QerryReturnType> {
        let rows = self
            .client
            .query(
                &self.commands.get_n_messages_before_statement,
                &[
                    &(channel_id as i64),
                    &(before_message_id as i64),
                    &(number_of_messages as i64),
                ],
            )
            .await?;
        Ok(QerryReturnType::Messages(
            data_types::Message::from_db_rows(rows)?,
        ))
    }

    #[allow(unused)]
    pub async fn add_user(&self, user: &data_types::User) -> Result<QerryReturnType> {
        let res = self
            .client
            .query(
                &self.commands.add_user_statement,
                &[&user.username, &user.pub_key.to_string()],
            )
            .await?;
        let row = res
            .get(0)
            .ok_or_else(|| anyhow::format_err!("Database error at adding user"))?;
        Ok(QerryReturnType::U64(row.try_get::<_, i64>(0)? as u64))
    }

    #[allow(unused)]
    pub async fn add_channel(&self, channel: &data_types::Channel) -> Result<QerryReturnType> {
        let res = self
            .client
            .query(&self.commands.add_channel_statement, &[&channel.name])
            .await?;
        let row = res
            .get(0)
            .ok_or_else(|| anyhow::format_err!("Database error at adding channel"))?;
        Ok(QerryReturnType::U64(row.try_get::<_, i64>(0)? as u64))
    }

    #[allow(unused)]
    pub async fn add_user_channel_link(
        //TODO: probably doesn't work. write tests
        &self,
        user_id: u64,
        channel_id: u64,
    ) -> Result<QerryReturnType> {
        let res = self
            .client
            .query(
                &self.commands.add_user_channel_link_statement,
                &[&(user_id as i64), &(channel_id as i64)],
            )
            .await?;
        let row = res
            .get(0)
            .ok_or_else(|| anyhow::format_err!("Database error at adding link"))?;
        Ok(QerryReturnType::U64(row.try_get::<_, i64>(0)? as u64))
    }

    #[allow(unused)]
    pub async fn get_user_channels(&self, user_id: u64) -> Result<QerryReturnType> {
        let res = self
            .client
            .query(
                &self.commands.get_user_channels_statement,
                &[&(user_id as i64)],
            )
            .await?;
        Ok(QerryReturnType::Channels(
            data_types::Channel::from_db_rows(res)?,
        ))
    }
}
