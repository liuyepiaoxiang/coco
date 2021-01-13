use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
    pub name: String,
    pub fist_commit: String,
    pub last_commit: String,
    pub duration: String,
    pub author: String,
}

impl Branch {
    pub fn new(name: &str) -> Branch {
        Branch {
            name: name.to_string(),
            fist_commit: "".to_string(),
            last_commit: "".to_string(),
            duration: "".to_string(),
            author: "".to_string(),
        }
    }
}
