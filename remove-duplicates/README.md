# 🧹 Remove Duplicates - Rust Web API

A simple web application built with Actix Web that removes duplicate lines from text files. This project demonstrates how to build a REST API with file upload/download capabilities in Rust.

## 🌟 Features

- Web interface for uploading text files
- Automatic detection and removal of duplicate lines
- Preserves original order of unique lines
- Download cleaned files with duplicates removed
- File size limit (2MB) enforcement
- Responsive HTML interface using MiniJinja templates

## 🛠️ Technology Stack

- **Rust 2024 Edition**
- **Actix Web** - Fast, powerful web framework
- **Actix Multipart** - For file upload handling
- **MiniJinja** - Templating engine for HTML responses
- **Tokio** - Asynchronous runtime

## 📋 Requirements

- Rust 1.75+
- Cargo

## 🚀 Running the Project

### Local Development

```bash
# Clone the repository (if you haven't already)
cd remove-duplicates

# Build and run in development mode
cargo run
```

The server will start at http://0.0.0.0:8080

### Docker Deployment

This project includes a Dockerfile for containerized deployment:

```bash
# Build Docker image
docker build -t remove-duplicates .

# Run container
docker run -p 8080:8080 remove-duplicates
```

## 📝 How It Works

1. Upload a text file through the web interface
2. The server processes the file, removing duplicate lines while preserving order
3. A results page displays the cleaned content with duplicate count
4. You can download the cleaned file from the results page

## 🧪 Implementation Details

- Uses HashSet for efficient duplicate detection
- Implements streaming file upload to handle large files efficiently
- Validates file type (only .txt allowed) and size
- Provides immediate feedback through the web interface

## 📚 Project Structure

```
./
├── src/
│   └── main.rs         # Application code
├── static/
│   └── index.html      # Web interface
├── templates/
│   └── result.html     # Results template
├── Cargo.toml          # Dependencies
└── Dockerfile          # Container definition
```

## 🔍 Future Improvements

- Add support for more file types
- Implement background processing for larger files
- Add user authentication
- Create API endpoints for programmatic access
