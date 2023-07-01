use super::data_types;
use super::database_commands;
use anyhow::{Ok, Result};

pub struct DbManager {
    commands: database_commands::DatabaseCommands,
}

impl DbManager {
    pub async fn new(client: &tokio_postgres::Client) -> Self {
        Self {
            commands: database_commands::DatabaseCommands::new(client).await,
        }
    }

    #[allow(unused)]
    pub async fn save_message(
        &self,
        message: &data_types::Message,
        client: &tokio_postgres::Client,
    ) -> Result<u64> {
        let res = client
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
        let row = res.get(0);
        match row {
            Some(row) => {
                let id: i64 = row.try_get(0)?;
                Ok(id as u64)
            }
            None => {
                anyhow::bail!("database error at saving a message")
            }
        }
    }

    #[allow(unused)]
    pub async fn get_new_messages(
        &self,
        channel_id: u64,
        last_message_id: u64,
        client: &tokio_postgres::Client,
    ) -> Result<Vec<data_types::Message>> {
        let rows = client
            .query(
                &self.commands.get_new_messages_statement,
                &[&(channel_id as i64), &(last_message_id as i64)],
            )
            .await?;

        Ok(data_types::Message::from_db_rows(rows))
    }

    #[allow(unused)]
    pub async fn get_last_n_messages(
        &self,
        channel_id: u64,
        number_of_messages: u8, //ye let's not allow big numbers
        client: &tokio_postgres::Client,
    ) -> Result<Vec<data_types::Message>> {
        let rows = client
            .query(
                &self.commands.get_last_n_messages_statement,
                &[&(channel_id as i64), &(number_of_messages as i64)],
            )
            .await?;
        let mut vec = data_types::Message::from_db_rows(rows);
        Ok(vec)
    }

    #[allow(unused)]
    pub async fn get_n_messages_before(
        &self,
        channel_id: u64,
        before_message_id: u64,
        number_of_messages: u8,
        client: &tokio_postgres::Client,
    ) -> Result<Vec<data_types::Message>> {
        let rows = client
            .query(
                &self.commands.get_n_messages_before_statement,
                &[
                    &(channel_id as i64),
                    &(before_message_id as i64),
                    &(number_of_messages as i64),
                ],
            )
            .await?;

        let vec = data_types::Message::from_db_rows(rows);
        Ok(vec)
    }

    #[allow(unused)]
    pub async fn add_user(
        &self,
        user: &data_types::User,
        client: &tokio_postgres::Client,
    ) -> Result<u64> {
        let res = client
            .query(
                &self.commands.add_user_statement,
                &[&user.username, &user.pub_key.to_string()],
            )
            .await?;
        let row = res.get(0);
        match row {
            Some(row) => {
                let id: i64 = row.try_get(0)?;
                Ok(id as u64)
            }
            None => {
                anyhow::bail!("database error at adding a user")
            }
        }
    }

    #[allow(unused)]
    pub async fn add_channel(
        &self,
        channel: &data_types::Channel,
        client: &tokio_postgres::Client,
    ) -> Result<u64> {
        let res = client
            .query(&self.commands.add_channel_statement, &[&channel.name])
            .await?;
        let row = res.get(0);
        match row {
            Some(row) => {
                let id: i64 = row.try_get(0)?;
                Ok(id as u64)
            }
            None => {
                anyhow::bail!("database error at adding a channel")
            }
        }
    }

    #[allow(unused)]
    pub async fn add_user_channel_link(
        //TODO: probably doesn't work. write tests
        &self,
        user_id: u64,
        channel_id: u64,
        client: &tokio_postgres::Client,
    ) -> Result<u64> {
        let res = client
            .query(
                &self.commands.add_user_channel_link,
                &[&(user_id as i64), &(channel_id as i64)],
            )
            .await?;
        let row = res.get(0);
        match row {
            Some(row) => {
                let id: i64 = row.try_get(0)?;
                Ok(id as u64)
            }
            None => {
                anyhow::bail!("database error at adding a user-channel link")
            }
        }
    }

    #[allow(unused)]
    pub async fn get_user_channels(
        &self,
        user_id: u64,
        client: &tokio_postgres::Client,
    ) -> Result<Vec<data_types::Channel>> {
        let res = client
            .query(&self.commands.get_user_channels, &[&(user_id as i64)])
            .await?;
        Ok(data_types::Channel::from_db_rows(res))
    }
}
