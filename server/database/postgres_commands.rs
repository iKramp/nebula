pub struct Command {
    pub command: String,
}

impl Command {
    pub fn select(columns: Vec<String>, table: &str) -> Self {
        Command {
            command: format!("SELECT {} FROM {}", columns.join(", "), table),
        }
    }

    pub fn insert(columns: &str, items: Vec<Vec<String>>, table: &str) -> Self {
        let mut elements = Vec::new();
        for item in items.into_iter() {
            elements.push(item.join(", "))
        }
        Command {
            command: format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table,
                columns,
                elements.join("), (")
            ),
        }
    }

    pub fn _where(mut self, condition: &str) -> Self {
        self.command
            .push_str(format!(" WHERE {condition}").as_str());
        self
    }

    pub fn returning(mut self, field: &str) -> Self {
        self.command
            .push_str(format!(" RETURNING {field}").as_str());
        self
    }
}
