pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub channel_id: u64,
    pub text: String,
    pub date_created: u64,
}

impl Message {
    #[allow(unused)]
    pub fn to_params(&mut self) -> Vec<String> {
        //doesn't work because tokio postgres is weird. i'll try to fix it so don't delete
        vec![
            self.user_id.to_string(),
            self.channel_id.to_string(),
            self.text.clone(),
            self.date_created.to_string(),
        ]
    }
}
