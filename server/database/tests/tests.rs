#[cfg(test)]
mod tests {

    use crate::database::tests::init_db::init_db::*;
    use anyhow::{Error, Result};
    use std::env;
    use tokio_postgres::{Client, NoTls};

    const TEST_DB: &str = "testdb";

    fn clear_db() {
        if let Err(e) = drop_db(TEST_DB) {
            panic!("{}", e);
        }
        if let Err(e) = create_db(TEST_DB) {
            panic!("{}", e);
        }
    }

    async fn get_client() -> tokio_postgres::Client {
        let args = format!("host=localhost user=postgres dbname = {}", TEST_DB);
        let f = tokio_postgres::connect(&args, NoTls).await;
        let (client, connection) = f.expect("couldn't connect");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_save_message() {
        clear_db();

        let client = get_client().await;

        populate_db(&client).await.unwrap();

        let command = crate::database::database_commands::ServerCommands::new(&client).await;

        let res = client
            .execute(
                &command.save_message_statement,
                &[&"1", &"1", &"test message", &"345263546"],
            )
            .await;

        if let Err(e) = res {
            panic!("{}", e)
        }
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_get_new_messages() {
        //TODO: this only tests if you can retrieve the data without errors, but doesn't yet test if the data is correct
        clear_db();

        let last_message_id = 2;
        let channel_id = 1;

        let client = get_client().await;

        populate_db(&client).await.unwrap();

        let command = crate::database::database_commands::ServerCommands::new(&client).await;

        let res = client
            .execute(&command.get_new_messages_statement, &[&"1", &"2"])
            .await;

        if let Err(e) = res {
            panic!("{}", e)
        }
    }
}
