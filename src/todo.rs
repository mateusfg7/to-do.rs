pub(crate) struct Todo {
    pub done: bool,
    pub task: String,
    pub time: Option<f32>,
}

impl Todo {
    pub(crate) fn serialize(&self) -> String {
        let marker = if self.done { 'x' } else { ' ' };

        format!(
            "[{}] {} ({})\n",
            marker,
            self.task,
            self.time.map_or("".to_string(), |f| format!("{:.1}", f))
        )
    }
}
