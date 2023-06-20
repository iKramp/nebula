use super::postgres_commands::Command;

pub fn save_message(user_id: u32, channel_id: u32, text: String, date_created: u64) -> String {
    let command = Command::insert(
        "user_id, channel_id, text, date_created",
        vec![vec![
            user_id.to_string(),
            channel_id.to_string(),
            text,
            date_created.to_string(),
        ]],
        "message",
    )
    .returning("id");
    command.command
}
