use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Note {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            tags,
            created_at: now,
            updated_at: now,
        }
    }
}

fn get_notes_dir() -> Result<PathBuf> {
    let mut path = crate::config::get_config_dir()?;
    path.push("notes");
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn get_note_path(id: &Uuid) -> Result<PathBuf> {
    let mut path = get_notes_dir()?;
    path.push(format!("{}.json", id));
    Ok(path)
}

pub fn save_note(note: &Note) -> Result<()> {
    let path = get_note_path(&note.id)?;
    let json = serde_json::to_string_pretty(note)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_all_notes() -> Result<Vec<Note>> {
    let notes_dir = get_notes_dir()?;
    let mut notes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(notes_dir) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(note) = serde_json::from_str::<Note>(&content) {
                        notes.push(note);
                    }
                }
            }
        }
    }
    
    // Sort by creation date, newest first
    notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    Ok(notes)
}

pub fn delete_note(id_prefix: &str) -> Result<()> {
    let all_notes = load_all_notes()?;
    
    // Find note by ID prefix
    let matching_notes: Vec<_> = all_notes
        .iter()
        .filter(|n| n.id.to_string().starts_with(id_prefix))
        .collect();
    
    match matching_notes.len() {
        0 => Err(anyhow!("No note found with ID starting with '{}'", id_prefix)),
        1 => {
            let note = matching_notes[0];
            let path = get_note_path(&note.id)?;
            fs::remove_file(path)?;
            Ok(())
        }
        _ => Err(anyhow!("Multiple notes found with ID starting with '{}'. Please be more specific.", id_prefix)),
    }
}