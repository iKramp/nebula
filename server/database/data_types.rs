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
