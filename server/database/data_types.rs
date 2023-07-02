use anyhow::Result;

pub trait FromDbRows {
    fn from_db_rows(rows: Vec<tokio_postgres::Row>) -> Result<Vec<Self>>
    where
        Self: Sized;
}

pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub channel_id: u64,
    pub text: String,
    pub date_created: u64,
}

impl Message {
    pub fn new(id: u64, user_id: u64, channel_id: u64, text: &str, date_created: u64) -> Self {
        Self {
            id,
            user_id,
            channel_id,
            text: text.to_owned(),
            date_created,
        }
    }
}

impl FromDbRows for Message {
    fn from_db_rows(message_rows: Vec<tokio_postgres::Row>) -> Result<Vec<Self>> {
        let mut message_vec: Vec<Message> = Vec::new();

        for row in message_rows {
            let id: i64 = row.try_get(0)?;
            let user_id: i64 = row.try_get(1)?;
            let channel_id: i64 = row.try_get(2)?;
            let text: String = row.try_get(3)?;
            let date_created: i64 = row.try_get(4)?;
            message_vec.push(Self::new(
                id as u64,
                user_id as u64,
                channel_id as u64,
                &text,
                date_created as u64,
            ));
        }
        Ok(message_vec)
    }
}

pub struct Channel {
    id: u64,
    pub name: String,
}

impl Channel {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
        }
    }
}

impl FromDbRows for Channel {
    fn from_db_rows(channel_rows: Vec<tokio_postgres::Row>) -> Result<Vec<Self>> {
        let mut channel_vec = Vec::new();

        for row in channel_rows {
            let id: i64 = row.try_get(0)?;
            let name: String = row.try_get(1)?;
            channel_vec.push(Self::new(id as u64, &name));
        }

        Ok(channel_vec)
    }
}

pub struct User {
    id: u64,
    pub username: String,
    pub pub_key: u64,
}

impl User {
    pub fn new(id: u64, username: &str, pub_key: u64) -> Self {
        Self {
            id,
            username: username.to_owned(),
            pub_key,
        }
    }
}

impl FromDbRows for User {
    fn from_db_rows(user_rows: Vec<tokio_postgres::Row>) -> Result<Vec<Self>> {
        let mut user_vec = Vec::new();

        for row in user_rows {
            let id: i64 = row.try_get(0)?;
            let username: String = row.try_get(1)?;
            let pub_key: i64 = row.try_get(2)?;
            user_vec.push(Self::new(id as u64, &username, pub_key as u64));
        }

        Ok(user_vec)
    }
}
