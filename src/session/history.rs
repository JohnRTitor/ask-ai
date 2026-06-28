use crate::client::types::ChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationHistory {
    pub messages: Vec<ChatMessage>,
    pub system_prompt: Option<String>,
}

impl Default for ConversationHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl ConversationHistory {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            system_prompt: None,
        }
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: content.to_string(),
        });
    }

    pub fn add_assistant_message(&mut self, content: &str) {
        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: content.to_string(),
        });
    }

    pub fn set_system_prompt(&mut self, prompt: &str) {
        self.system_prompt = Some(prompt.to_string());
    }

    pub fn get_messages(&self) -> Vec<ChatMessage> {
        let mut all_msgs = Vec::new();

        if let Some(sys) = &self.system_prompt {
            all_msgs.push(ChatMessage {
                role: "system".to_string(),
                content: sys.clone(),
            });
        }

        all_msgs.extend(self.messages.clone());
        all_msgs
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
