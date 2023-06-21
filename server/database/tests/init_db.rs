use std::process::Command;

pub fn drop_db(db_name: &str) -> anyhow::Result<()> {
    Command::new("dropdb")
        .arg("--if-exists")
        .arg(db_name)
        .status()?;
    Ok(())
}

pub fn create_db(db_name: &str) -> anyhow::Result<()> {
    Command::new("createdb")
        .arg(db_name)
        .status()?;
    
    Ok(())
}