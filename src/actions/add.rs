use crate::todo::Todo;

pub fn add(task: &str) -> String {
    Todo {
        done: false,
        task: task.to_string(),
        time: None,
    }
    .serialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let task = "test";
        let expected = "[ ] test ()\n".to_string();

        assert_eq!(add(task), expected);
    }
}
