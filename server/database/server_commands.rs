use super::postgres_commands::Command;

pub fn add_message(user_id: u32, channel_id: u32, text: String, date_created: u64) -> String {
    let command = Command::insert(
        "user_id, channel_id, text, date_created".to_owned(),
        vec![vec![
            user_id.to_string(),
            channel_id.to_string(),
            text,
            date_created.to_string(),
        ]],
        "message".to_owned(),
    );
    command.command
}
