# Administrative Assistant

A personal administrative assistant for your terminal, written in pure Rust.

## Quick Start

```bash
# Clone the repository
git clone <https://github.com/Myrmecology/Administrative-Assistant.git>
cd admin-assist

# Build the project
cargo build --release

# Run cargo check to verify
cargo check

# Run the application
cargo run -- init

# Initialize the application
admin-assist init

# Create a new note
admin-assist note new "Meeting Notes" -c "Discuss project timeline" -t "work,important"

# List all notes
admin-assist note list

# List notes by tag
admin-assist note list -t work

# Search notes
admin-assist note search "project"

# Delete a note (use first 8 characters of ID)
admin-assist note delete 12345678

# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check for errors without building
cargo check

# Run tests
cargo test

admin-assist/
├── Cargo.toml
├── src/
│   ├── main.rs      # CLI entry point
│   ├── notes.rs     # Notes functionality
│   └── config.rs    # Configuration management

