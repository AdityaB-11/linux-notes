use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
        self.completed_at = if self.completed {
            Some(Utc::now())
        } else {
            None
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppData {
    pub notes: Vec<Note>,
    pub todos: Vec<TodoItem>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            notes: Vec::new(),
            todos: Vec::new(),
        }
    }
}

impl AppData {
    pub fn load() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("linux-notes");
        
        fs::create_dir_all(&data_dir).ok();
        
        let data_file = data_dir.join("data.json");
        
        if data_file.exists() {
            if let Ok(content) = fs::read_to_string(&data_file) {
                if let Ok(data) = serde_json::from_str(&content) {
                    return data;
                }
            }
        }
        
        Self::default()
    }

    pub fn save(&self) {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("linux-notes");
        
        fs::create_dir_all(&data_dir).ok();
        
        let data_file = data_dir.join("data.json");
        
        if let Ok(content) = serde_json::to_string_pretty(self) {
            fs::write(&data_file, content).ok();
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn remove_note(&mut self, id: &Uuid) {
        self.notes.retain(|note| note.id != *id);
    }

    pub fn add_todo(&mut self, todo: TodoItem) {
        self.todos.push(todo);
    }

    pub fn remove_todo(&mut self, id: &Uuid) {
        self.todos.retain(|todo| todo.id != *id);
    }

    pub fn toggle_todo(&mut self, id: &Uuid) {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == *id) {
            todo.toggle_completed();
        }
    }
}
