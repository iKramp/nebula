#[cfg(test)]
mod tests {

    use crate::database::tests::init_db::init_db::*;
    use crate::database::{data_types, database_actions};
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
        //TODO: merge this and the normal connect_to_db functions. maybe change what is hardcoded and what is a parameter
        let args = format!(
                "host= user= password= dbname = {}",
            TEST_DB
        );
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
        //might remove because the next test tests both

        let client = get_client().await;

        setup_db(&client).await.unwrap();
        populate_db(&client).await.unwrap();

        let command = crate::database::database_commands::DatabaseCommands::new(&client).await;

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

        let client = get_client().await;

        setup_db(&client).await.unwrap();
        populate_db(&client).await.unwrap();

        let db_manager = database_actions::DbManager::new(&client).await;

        let message_1 = data_types::Message::new(1, 1, 1, "foo", 9741985714305981);
        let message_2 = data_types::Message::new(1, 2, 1, "bar", 9741985714306934);

        db_manager.save_message(&message_1, &client).await.unwrap();
        db_manager.save_message(&message_2, &client).await.unwrap();

        let test_vec = vec![message_1, message_2];

        let returned_vec = db_manager.get_new_messages(1, 5, &client).await.unwrap();

        assert_eq!(test_vec.len(), returned_vec.len());
        for ea in test_vec.iter().enumerate() {
            let eb = returned_vec.get(ea.0).unwrap();
            //not asserting IDs because the DB assigns them automatically. they are managed by the DB and cannot be known before saving the messages
            assert_eq!(ea.1.channel_id, eb.channel_id);
            assert_eq!(ea.1.user_id, eb.user_id);
            assert_eq!(ea.1.text, eb.text);
            assert_eq!(ea.1.date_created, eb.date_created)
        }
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

        assert_eq!(test_vec.len(), returned_vec.len());
        for ea in test_vec.iter().enumerate() {
            let eb = returned_vec.get(ea.0).unwrap();
            //not asserting IDs because the DB assigns them automatically. they are managed by the DB and cannot be known before saving the messages
            assert_eq!(ea.1.channel_id, eb.channel_id);
            assert_eq!(ea.1.user_id, eb.user_id);
            assert_eq!(ea.1.text, eb.text);
            assert_eq!(ea.1.date_created, eb.date_created)
        }
    }

    #[tokio::test]
    #[ignore]
    async fn database_test_get_n_messages_before() {
        //clear_db();

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

        assert_eq!(test_vec.len(), returned_vec.len());
        for ea in test_vec.iter().enumerate() {
            let eb = returned_vec.get(ea.0).unwrap();
            //not asserting IDs because the DB assigns them automatically. they are managed by the DB and cannot be known before saving the messages
            assert_eq!(ea.1.channel_id, eb.channel_id);
            assert_eq!(ea.1.user_id, eb.user_id);
            assert_eq!(ea.1.text, eb.text);
            assert_eq!(ea.1.date_created, eb.date_created)
        }
    }
}
