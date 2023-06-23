pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub channel_id: u64,
    pub text: String,
    pub date_created: u64,
}

impl Message {
    pub fn new(id: u64, user_id: u64, channel_id: u64, text: &str, date_created: u64) -> Self {
        Message {
            id,
            user_id,
            channel_id,
            text: text.to_owned(),
            date_created,
        }
    }

    #[allow(unused)]
    pub fn gen_params(&mut self) -> Vec<String> {
        //doesn't work because tokio postgres is weird. i'll try to fix it so don't delete
        vec![
            self.user_id.to_string(),
            self.channel_id.to_string(),
            self.text.clone(),
            self.date_created.to_string(),
        ]
    }
}
