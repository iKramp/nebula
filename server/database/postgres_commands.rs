pub struct Command {
    pub command: String,
}

#[allow(dead_code)] //will be used, this is just to remind me not to forget the ; at the end
impl Command {
    pub fn execute(&mut self) {
        self.command.push(';');
        //execute here
    }

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

    pub fn _where(mut self, condition: Vec<&str>) -> Self {
        self.command
            .push_str(format!(" WHERE {}", condition.join(" AND ")).as_str());
        self
    }

    pub fn returning(mut self, field: &str) -> Self {
        self.command
            .push_str(format!(" RETURNING {field}").as_str());
        self
    }
}
