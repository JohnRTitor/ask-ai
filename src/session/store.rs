use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::session::history::ConversationHistory;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStore {
    pub history: ConversationHistory,
    pub model: String,
    pub timestamp: String,
}

pub fn get_sessions_dir() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("ask-ai");
    path.push("sessions");
    path
}

pub fn save_session(id: &str, model: &str, history: &ConversationHistory) -> Result<()> {
    let dir = get_sessions_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let mut path = dir;
    path.push(format!("{}.json", id));

    let store = SessionStore {
        history: history.clone(),
        model: model.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    let json = serde_json::to_string_pretty(&store)?;
    std::fs::write(&path, json).with_context(|| format!("Failed to save session to {:?}", path))?;
    Ok(())
}

pub fn load_session(id: &str) -> Result<SessionStore> {
    let mut path = get_sessions_dir();
    path.push(format!("{}.json", id));

    let json = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read session from {:?}", path))?;
    
    let store: SessionStore = serde_json::from_str(&json)?;
    Ok(store)
}
