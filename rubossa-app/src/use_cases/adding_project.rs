#[derive(Debug, Clone)]
pub struct AddProject {
    name: String,
}

impl Default for AddProject {
    fn default() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_project() {
        let cmd = AddProject {
            name: "project 1".to_string(),
            ..Default::default()
        };
    }
}
