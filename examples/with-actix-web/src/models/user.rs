use std::time::SystemTime;

#[derive(Debug)]
pub struct User {
    name: String,
    last_login: SystemTime,
}

impl User {
    pub fn new(name: &str) -> User {
        User {
            name: name.to_string(),
            last_login: SystemTime::now(),
        }
    }

    pub fn get_message(&self) -> String {
        let duration = SystemTime::now().duration_since(self.last_login);
        match duration {
            Ok(n) => format!(
                "Hi {}! It's been {} seconds since we last met!",
                self.name,
                n.as_secs()
            ),
            _ => format!("Hi {}! Pleased to meet you!", self.name),
        }
    }
}
