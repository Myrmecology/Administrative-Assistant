# Administrative Assistant

A powerful command-line personal assistant written in pure Rust. Manage notes, monitor system health, and streamline your daily tasks - all from your terminal.

## Features

### 📝 Notes Management
- **Create** notes with titles, content, and tags
- **Search** through all notes with full-text search
- **List** notes with optional tag filtering
- **Delete** notes using partial ID matching
- **Organize** with tag-based categorization

### 💻 System Monitoring
- **System Snapshot** - Quick overview of system health
- **Resource Usage** - CPU, memory, and process monitoring
- **System Information** - OS details and hardware specs

## Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/Myrmecology/Administrative-Assistant.git
cd admin-assist

# Build the project
cargo build --release

# The binary will be available at ./target/release/admin-assist

# Install directly using cargo
cargo install --path .

Initialize the Application
admin-assist init

This creates the configuration directory and prepares the application for first use.
Notes Commands
Create a Note

# With all parameters
admin-assist note new "Meeting Notes" -c "Discuss Q4 targets" -t "work,important"

# Interactive mode (multi-line content)
admin-assist note new "Shopping List"
# Then type your content, press Enter twice to save

List Notes
# List all notes
admin-assist note list

# Filter by tag
admin-assist note list -t work

Search Notes
# Search in titles, content, and tags
admin-assist note search "project"

Delete a Note
# Delete using first 8 characters of ID
admin-assist note delete 206c3960

System Commands
System Snapshot
# View current system status
admin-assist snapshot

Shows:

System and OS information
CPU usage and core count
Memory usage with percentage
Running process count

Command Reference
admin-assist <COMMAND>

Commands:
  init      Initialize configuration
  note      Manage notes
  snapshot  Show system snapshot
  help      Print help information

Note Subcommands:
  new       Create a new note
  list      List all notes
  search    Search notes
  delete    Delete a note by ID

  Examples
Daily Workflow
# Start your day
admin-assist snapshot  # Check system health

# Create a todo list
admin-assist note new "Today's Tasks" -t "daily,todo"

# Quick reminder
admin-assist note new "Call client" -c "Discuss contract renewal" -t "urgent"

# Review all urgent items
admin-assist note list -t urgent

# Search for specific topics
admin-assist note search "contract"

Note Organization
# Create categorized notes
admin-assist note new "Project Ideas" -t "ideas,development"
admin-assist note new "Bug Report" -t "bugs,urgent,frontend"
admin-assist note new "Meeting Minutes" -t "meetings,2024"

# Filter by category
admin-assist note list -t ideas
admin-assist note list -t urgent

Data Storage

Configuration: %APPDATA%\admin-assist\AdminAssist\config (Windows)
Notes: Stored as JSON files in the config directory
Backup: Simply copy the config directory to backup all notes

Development
Project Structure
admin-assist/
├── Cargo.toml           # Project configuration
├── src/
│   ├── main.rs          # CLI entry point
│   ├── notes.rs         # Notes functionality
│   ├── snapshot.rs      # System monitoring
│   └── config.rs        # Configuration management

Running Tests
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

Adding New Features

Create a new module in src/
Add the module declaration in main.rs
Add new commands to the Commands enum
Implement the feature logic

Roadmap

 Task scheduler with cron-like syntax
 File organizer with custom rules
 Note encryption
 Export notes to Markdown/PDF
 Note templates
 Quick calculations
 Integration with external calendars
 Backup and sync functionality

Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

Fork the repository
Create your feature branch (git checkout -b feature/AmazingFeature)
Commit your changes (git commit -m 'Add some AmazingFeature')
Push to the branch (git push origin feature/AmazingFeature)
Open a Pull Request

License
This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgments

Built with Rust
CLI parsing with clap
Terminal colors with colored
System information via sysinfo


Admin Assist - Your terminal, your assistant, your way.

# Administrative Assistant 🦀

Built with Rust 🦀
Happy coding

