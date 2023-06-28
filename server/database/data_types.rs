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

pub struct Channel {
    id: u64,
    name: String,
}

impl Channel {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}

pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> Self {
        Self { id, username }
    }
}