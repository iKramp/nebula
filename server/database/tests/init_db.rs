#[cfg(test)]
pub mod init_db {
    use std::fs;
    use std::process::Command;

    pub fn drop_db(db_name: &str) -> anyhow::Result<()> {
        Command::new("dropdb")
            .arg("--if-exists")
            .arg(db_name)
            .status()?;
        Ok(())
    }

    pub fn create_db(db_name: &str) -> anyhow::Result<()> {
        Command::new("createdb").arg(db_name).status()?;

        Ok(())
    }

    pub fn get_populate_db_commands() -> anyhow::Result<String> {
        let file = include_bytes!("../init_db_commands.txt");
        let file_content = std::str::from_utf8(file)?.to_owned();
        Ok(file_content)
    }

    pub async fn populate_db(client: &tokio_postgres::Client) -> anyhow::Result<()> {
        match super::init_db::get_populate_db_commands() {
            Ok(str) => {
                let commands = str.split(';');

                for command in commands {
                    let res = client.execute(command, &[]).await;
                    match res {
                        Err(e) => {
                            panic!("{}", e);
                        }
                        Ok(res) => {
                            println!("{}", res);
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
