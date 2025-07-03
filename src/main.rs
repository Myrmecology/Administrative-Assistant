mod notes;
mod config;

use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "admin-assist")]
#[command(about = "A personal administrative assistant for your terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage notes
    Note {
        #[command(subcommand)]
        action: NoteCommands,
    },
    /// Initialize configuration
    Init,
}

#[derive(Subcommand)]
enum NoteCommands {
    /// Create a new note
    New {
        /// Note title
        title: String,
        /// Note content
        #[arg(short, long)]
        content: Option<String>,
        /// Tags (comma separated)
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// List all notes
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },
    /// Search notes
    Search {
        /// Search query
        query: String,
    },
    /// Delete a note by ID
    Delete {
        /// Note ID (first few characters)
        id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Ensure config directory exists
    config::ensure_config_dir()?;

    match cli.command {
        Commands::Note { action } => handle_note_command(action),
        Commands::Init => {
            println!("{}", "Administrative Assistant initialized successfully!".green());
            println!("Configuration directory: {}", config::get_config_dir()?.display());
            Ok(())
        }
    }
}

fn handle_note_command(cmd: NoteCommands) -> Result<()> {
    match cmd {
        NoteCommands::New { title, content, tags } => {
            let tags_vec = tags
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();
            
            let content = content.unwrap_or_else(|| {
                println!("Enter note content (press Enter twice to finish):");
                let mut lines = Vec::new();
                let mut empty_lines = 0;
                
                loop {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).unwrap();
                    
                    if line.trim().is_empty() {
                        empty_lines += 1;
                        if empty_lines >= 2 {
                            break;
                        }
                    } else {
                        empty_lines = 0;
                    }
                    
                    lines.push(line);
                }
                
                lines.join("")
            });
            
            let note = notes::Note::new(title, content, tags_vec);
            notes::save_note(&note)?;
            
            println!("{} {}", "✓".green(), "Note created successfully!");
            println!("ID: {}", note.id.to_string()[..8].bright_blue());
        }
        
        NoteCommands::List { tag } => {
            let all_notes = notes::load_all_notes()?;
            let filtered_notes: Vec<_> = if let Some(tag_filter) = tag {
                all_notes.into_iter()
                    .filter(|n| n.tags.contains(&tag_filter))
                    .collect()
            } else {
                all_notes
            };
            
            if filtered_notes.is_empty() {
                println!("{}", "No notes found.".yellow());
            } else {
                println!("{}", format!("Found {} note(s):", filtered_notes.len()).bright_white());
                println!("{}", "─".repeat(60));
                
                for note in filtered_notes {
                    println!("{} {}", 
                        note.id.to_string()[..8].bright_blue(),
                        note.title.bright_white()
                    );
                    println!("  {} {}", "Created:".dim(), note.created_at.format("%Y-%m-%d %H:%M"));
                    if !note.tags.is_empty() {
                        println!("  {} {}", "Tags:".dim(), note.tags.join(", ").cyan());
                    }
                    println!("  {} {}", "Content:".dim(), 
                        note.content.lines().next().unwrap_or("").chars().take(50).collect::<String>()
                    );
                    println!("{}", "─".repeat(60));
                }
            }
        }
        
        NoteCommands::Search { query } => {
            let all_notes = notes::load_all_notes()?;
            let query_lower = query.to_lowercase();
            
            let matching_notes: Vec<_> = all_notes.into_iter()
                .filter(|n| {
                    n.title.to_lowercase().contains(&query_lower) ||
                    n.content.to_lowercase().contains(&query_lower) ||
                    n.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                })
                .collect();
            
            if matching_notes.is_empty() {
                println!("{}", "No notes found matching your search.".yellow());
            } else {
                println!("{}", format!("Found {} matching note(s):", matching_notes.len()).bright_white());
                println!("{}", "─".repeat(60));
                
                for note in matching_notes {
                    println!("{} {}", 
                        note.id.to_string()[..8].bright_blue(),
                        note.title.bright_white()
                    );
                    if !note.tags.is_empty() {
                        println!("  {} {}", "Tags:".dim(), note.tags.join(", ").cyan());
                    }
                    println!("{}", "─".repeat(60));
                }
            }
        }
        
        NoteCommands::Delete { id } => {
            notes::delete_note(&id)?;
            println!("{} {}", "✓".green(), "Note deleted successfully!");
        }
    }
    
    Ok(())
}
