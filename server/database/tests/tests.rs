#[cfg(test)]
mod tests {

    use crate::database::tests::init_db::init_db::*;
    use crate::database::{data_types, database_actions};
    use anyhow::{Error, Result};
    use std::env;
    use tokio_postgres::{Client, NoTls};

    const TEST_DB: &str = "testdb";

    async fn get_client() -> tokio_postgres::Client {
        //TODO: merge this and the normal connect_to_db functions. maybe change what is hardcoded and what is a parameter
        let args = std::env::var("DB_CONNECT_ARGS").unwrap();
        let f = tokio_postgres::connect(&args, NoTls).await;
        let (client, connection) = f.expect("couldn't connect");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
    }

    async fn assert_equal_vectors(first_vec: Vec<data_types::Message>, second_vec: Vec<data_types::Message>) {
        assert_eq!(first_vec.len(), second_vec.len());

        for (index, message) in first_vec.iter().enumerate() {
            let other_message = second_vec.get(index).unwrap();

            assert_eq!(message.channel_id, other_message.channel_id);
            assert_eq!(message.user_id, other_message.user_id);
            assert_eq!(message.text, other_message.text);
            assert_eq!(message.date_created, other_message.date_created);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_save_messages() {
            let client = get_client().await;

            setup_db(&client).await.unwrap();
            populate_db(&client).await.unwrap();

            let db_manager = database_actions::DbManager::new(&client).await;

            let message_1 = data_types::Message::new(1, 1, 1, "foo", 9741985714305981);
            let message_2 = data_types::Message::new(1, 2, 1, "bar", 9741985714306934);

            let id_1 = db_manager.save_message(&message_1, &client).await.unwrap();
            let id_2 = db_manager.save_message(&message_2, &client).await.unwrap();

            assert_eq!(id_1, 6);
            assert_eq!(id_2, 7);
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_get_new_messages() {
        let client = get_client().await;

        setup_db(&client).await.unwrap();
        populate_db(&client).await.unwrap();

        let db_manager = database_actions::DbManager::new(&client).await;

        let message_1 = data_types::Message::new(1, 2, 1, "Random text 4", 1687249040004);
        let message_2 = data_types::Message::new(1, 1, 1, "Random text 5", 1687249040005);

        let test_vec = vec![message_1, message_2];

        let returned_vec = db_manager.get_new_messages(1, 3, &client).await.unwrap();

        assert_equal_vectors(test_vec, returned_vec).await;
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_get_last_n_messages() {
        let client = get_client().await;

        setup_db(&client).await.unwrap();
        populate_db(&client).await.unwrap();

        let db_manager = database_actions::DbManager::new(&client).await;

        let test_vec = vec![
            data_types::Message::new(3, 1, 1, "Random text 3", 1687249040003),
            data_types::Message::new(4, 2, 1, "Random text 4", 1687249040004),
            data_types::Message::new(5, 1, 1, "Random text 5", 1687249040005),
        ];

        let mut returned_vec = db_manager.get_last_n_messages(1, 3, &client).await.unwrap();
        returned_vec.reverse();

        assert_equal_vectors(returned_vec, test_vec).await;
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_get_n_messages_before() {
        let client = get_client().await;

        setup_db(&client).await.unwrap();
        populate_db(&client).await.unwrap();

        let db_manager = database_actions::DbManager::new(&client).await;

        let test_vec = vec![
            data_types::Message::new(2, 2, 1, "Random text 2", 1687249040002),
            data_types::Message::new(3, 1, 1, "Random text 3", 1687249040003),
            data_types::Message::new(4, 2, 1, "Random text 4", 1687249040004),
        ];

        let mut returned_vec = db_manager
            .get_n_messages_before(1, 5, 3, &client)
            .await
            .unwrap();
        returned_vec.reverse();

        assert_equal_vectors(test_vec, returned_vec).await;
    }
}
