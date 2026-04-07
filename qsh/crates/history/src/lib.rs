pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub filename: String,
    pub size: String,
    pub date: String,
    pub status: String,
}

impl Note {
    pub async fn to_string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}",
            self.user_id, self.filename, self.size, self.date, self.status
        )
    }
}
