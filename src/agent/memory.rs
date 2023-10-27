use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageSender {
    User,
    Bot,
}

impl ToString for MessageSender {
    fn to_string(&self) -> String {
        match self {
            Self::User => "User".to_owned(),
            Self::Bot => "You".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContent {
    Message {
        sender: MessageSender,
        content: String,
    },
}

impl MemoryContent {
    pub fn path(&self) -> String {
        match self {
            Self::Message { .. } => "agent/memory/Message".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: u64,
    pub content: MemoryContent,
}

pub struct MemoryStore;

impl MemoryStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn save_memory(&self, memory: &Memory) {
        let path = memory.content.path();
        let _ = fs::create_dir_all(&path);
        let memory_string = serde_json::to_string_pretty(memory).unwrap();
        let _ = fs::write(format!("{}/{}.json", path, memory.id), memory_string);
    }

    fn save_message(&self, content: &String, sender: MessageSender) {
        let path = "agent/memory/Message";
        let _ = fs::create_dir_all(&path);

        let id = fs::read_dir(path).unwrap().count() as u64;
        let memory = Memory {
            id,
            content: MemoryContent::Message {
                sender,
                content: content.clone(),
            },
        };
        self.save_memory(&memory);
    }

    pub fn save_user_message(&self, content: &String) {
        self.save_message(content, MessageSender::User)
    }

    pub fn save_bot_message(&self, content: &String) {
        self.save_message(content, MessageSender::Bot)
    }

    pub fn get_message(&self, id: u64) -> Result<Memory, String> {
        let path = format!("agent/memory/Message/{}.json", id);

        match fs::read_to_string(path) {
            Ok(s) => Ok(serde_json::from_str(s.as_str()).unwrap()),
            Err(_) => Err(format!("Message {} does not exist", id)),
        }
    }

    pub fn get_last_n_messages(&self, n: usize) -> Vec<Memory> {
        let path = "agent/memory/Message";
        let num_messages = fs::read_dir(path).unwrap().count();
        if num_messages == 0 {
            return Vec::new();
        }

        let max_id = num_messages as isize;
        let min_id = 0.max(max_id - n as isize);

        let mut messages = Vec::new();
        for id in min_id..max_id {
            let message = self.get_message(id as u64).unwrap();
            messages.push(message);
        }

        messages
    }
}