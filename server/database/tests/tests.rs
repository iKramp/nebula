use super::init_db;

#[cfg(test)]
mod tests {
    #[test]
    fn clear_db() {
        if let Err(e) = super::init_db::drop_db("testdb") {
            panic!("{}", e);
        }
        if let Err(e) = super::init_db::create_db("testdb") {
            panic!("{}", e);
        }
    }
}