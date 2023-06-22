use super::postgres_commands::Command;

pub fn save_message(user_id: u32, channel_id: u32, text: String, date_created: u64) -> Command {
    Command::insert(
        "user_id, channel_id, text, date_created",
        vec![vec![
            user_id.to_string(),
            channel_id.to_string(),
            format!("'{}'", text),
            date_created.to_string(),
        ]],
        "messages",
    )
    .returning("id")
}

pub fn get_new_messages(channel_id: u32, last_message_id: u32) -> Command {
    Command::select(
        vec![
            "id".to_owned(),
            "user_id".to_owned(),
            "text".to_owned(),
            "date_created".to_owned(),
        ],
        "messages",
    )
    ._where(vec![
        &format!("messages.channel_id = {channel_id}"),
        &format!("messages.id > {last_message_id}"),
    ])
}
