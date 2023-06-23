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
        let file = include_bytes!("../populate_db_commands.txt");
        let file_content = std::str::from_utf8(file)?.to_owned();
        Ok(file_content)
    }

    pub fn get_setup_db_commands() -> anyhow::Result<String> {
        let file = include_bytes!("../setup_db_commands.txt");
        let file_content = std::str::from_utf8(file)?.to_owned();
        Ok(file_content)
    }

    pub async fn populate_db(client: &tokio_postgres::Client) -> anyhow::Result<()> {
        let str = super::init_db::get_populate_db_commands()?;

        let commands = str.split(';');

        for command in commands {
            let _res = client.execute(command, &[]).await?;
        }
        Ok(())
    }

    pub async fn setup_db(client: &tokio_postgres::Client) -> anyhow::Result<()> {
        let str = super::init_db::get_setup_db_commands()?;

        let commands = str.split(';');

        for command in commands {
            let _res = client.execute(command, &[]).await?;
        }
        Ok(())
    }
}
