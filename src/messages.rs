use anyhow::bail;
use anyhow::Result;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

lazy_static! {
    static ref MESSAGES: MessagesData = load_messages();
}

#[derive(Deserialize, Debug)]
struct MessagesData {
    command_description: Vec<CommandDescription>,
}

#[derive(Deserialize, Debug)]
struct CommandDescription {
    name: String,
    description: String,
    help: String,
}

impl CommandDescription {
    pub fn to_string(&self) -> String {
        format!(
            "```md
# {}
## Description
{}
## Usage
{}
```",
            self.name, self.description, self.help
        )
    }
}

fn load_messages() -> MessagesData {
    let path = Path::new("resources/messages.toml");
    let mut file = File::open(&path).expect("Failed to open a file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to convert to string");

    toml::from_str(&content).expect("Failed to deserialize")
}

pub fn find_message(command_name: &str) -> Result<String> {
    MESSAGES
        .command_description
        .iter()
        .find(|&description| description.name == command_name)
        .map_or_else(|| bail!("unknown command name"), |c| Ok(c.to_string()))
}
