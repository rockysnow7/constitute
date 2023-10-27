pub mod memory;

extern crate strfmt;

use memory::{MemoryStore, MemoryContent, MessageSender};
use serde::{Serialize, Deserialize};
use serde_json::{self, json};
use std::{
    collections::HashMap,
    fs,
};
use strfmt::Format;

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentConfig {
    pub name: String,
}

pub struct Agent {
    config: AgentConfig,
    memory_store: MemoryStore,
}

impl Agent {
    pub fn new(config: AgentConfig) -> Self {
        let memory_store = MemoryStore::new();

        Self { config, memory_store }
    }

    pub fn get() -> Result<Self, String> {
        let config_string = match fs::read_to_string("agent/config.json") {
            Ok(content) => content,
            Err(_) => return Err("Assistant does not exist".to_owned()),
        };

        let config: AgentConfig = serde_json::from_str(&config_string).unwrap();

        Ok(Self::new(config))
    }

    pub fn create(config: &AgentConfig) {
        let _ = fs::create_dir("agent");

        let config_string = serde_json::to_string_pretty(&config).unwrap();
        let _ = fs::write("agent/config.json".to_owned(), config_string);
    }

    fn get_conversation(&self) -> String {
        let messages = self.memory_store.get_last_n_messages(10);
        //let MemoryContent::Message { content, .. } = messages[0].content.clone();
        let mut conversation = String::new();
        for message in messages {
            let MemoryContent::Message { sender, content } = message.content;
            conversation.extend(format!("{}: {}\n", sender.to_string(), content).chars());
        }
        conversation.pop();

        conversation
    }

    fn build_prompt(&self) -> String {
        let mut template_vars = HashMap::new();
        template_vars.insert("conversation".to_owned(), self.get_conversation());

        let template = fs::read_to_string("prompts/main.txt").unwrap();
        let prompt = template.format(&template_vars).unwrap();

        prompt
    }

    async fn get_llm_response(&self) -> String {
        let prompt = self.build_prompt();
        println!("{prompt}");

        let client = reqwest::Client::new();
        let response = client
            .post("http://127.0.0.1:5000/predict")
            .json(&json!({
                "prompt": prompt,
            }))
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let response_value = response
            .get("response")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

        response_value
    }

    pub async fn respond(&self, user_message: &String) -> String {
        self.memory_store.save_user_message(user_message);
        let response_content = self.get_llm_response().await;
        self.memory_store.save_bot_message(&response_content);

        response_content
    }
}