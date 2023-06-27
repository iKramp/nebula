use super::data_types;
use super::database_commands;
use anyhow::{Ok, Result};

fn get_message_vec(message_rows: Vec<tokio_postgres::Row>) -> Vec<data_types::Message> {
    let mut message_vec = Vec::new();

    //this can panic, but it should if anything is wrong so we know immediately
    for row in message_rows {
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
        ));
    }
    message_vec
}

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
        let id: i64 = res.get(0 as usize).unwrap().get(0 as usize);
        Ok(id as u64)
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

        Ok(get_message_vec(rows))
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
        let mut vec = get_message_vec(rows);
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

        let vec = get_message_vec(rows);
        Ok(vec)
    }

    #[allow(unused)]
    pub async fn add_user(&self, username: &str, client: &tokio_postgres::Client) -> Result<u64> {
        //TODO: probably doesn't work. write tests
        let res = client
            .query(&self.commands.add_user_statement, &[&username])
            .await?;
        let id: i64 = res.get(0 as usize).unwrap().get(0 as usize);
        Ok(id as u64)
    }

    #[allow(unused)]
    pub async fn add_channel(
        //TODO: probably doesn't work. write tests
        &self,
        name: &str,
        client: &tokio_postgres::Client,
    ) -> Result<u64> {
        let res = client
            .query(&self.commands.add_channel_statement, &[&name])
            .await?;
        let id: i64 = res.get(0 as usize).unwrap().get(0 as usize);
        Ok(id as u64)
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
        let id: i64 = res.get(0 as usize).unwrap().get(0 as usize);
        Ok(id as u64)
    }
}
