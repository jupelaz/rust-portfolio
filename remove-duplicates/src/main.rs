use std::collections::HashSet;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::NamedFile;
use actix_multipart::Multipart;

use futures_util::{StreamExt, TryStreamExt};
use minijinja::{Environment, context};

const MAX_FILE_SIZE: usize = 2 * 1024 * 1024; // 2 MB

async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn upload_file(mut payload: Multipart, tmpl: web::Data<Environment<'_>>) -> impl Responder {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename =
            match field.content_disposition().and_then(|cd| cd.get_filename()) {
                Some(name) => name.to_string(),
                None => "".to_string(),
            };
        if !filename.ends_with(".txt") {
            return HttpResponse::BadRequest().body("Only .txt files are allowed");
        }

        let mut content = Vec::new();

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(_) => return HttpResponse::BadRequest().body("Error reading file"),
            };
            if content.len() + data.len() > MAX_FILE_SIZE {
                return HttpResponse::BadRequest().body("File too large");
            }
            content.extend_from_slice(&data);
        }

        let input = String::from_utf8_lossy(&content);
        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if seen.insert(line) && !line.is_empty() {
                result.push(line.to_string());
            }
        }

        let output = result.join("\n");

        let tmpl = tmpl.get_template("result.html").unwrap();
        let html = tmpl.render(context! { content => output, count => result.len() }).unwrap();

        return HttpResponse::Ok()
            .content_type("text/html")
            .body(html);
    }

    HttpResponse::BadRequest().body("No file received")
}

async fn download(form: web::Form<std::collections::HashMap<String, String>>) -> impl Responder {
    if let Some(content) = form.get("content") {
        return HttpResponse::Ok()
            .append_header(("Content-Disposition", "attachment; filename=\"cleaned.txt\""))
            .body(content.clone());
    }

    HttpResponse::BadRequest().body("Download failed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server in http://127.0.0.1:8080");

    // Minijinja template engine
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("templates"));

    let tmpl_data = web::Data::new(env);

    HttpServer::new(move || {
        App::new()
            .app_data(tmpl_data.clone())
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_file))
            .route("/download", web::post().to(download))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to simulate the duplicate removal logic
    fn remove_duplicates(input: &str) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        
        for line in input.lines() {
            let line = line.trim();
            if seen.insert(line) && !line.is_empty() {
                result.push(line.to_string());
            }
        }
        
        result
    }
    
    #[tokio::test]
    async fn test_remove_duplicates_basic() {
        let input = "line1\nline2\nline1\nline3";
        let result = remove_duplicates(input);
        assert_eq!(result, vec!["line1", "line2", "line3"]);
    }
    
    #[tokio::test]
    async fn test_remove_duplicates_with_empty_lines() {
        let input = "line1\n\nline2\n\nline1\nline3";
        let result = remove_duplicates(input);
        assert_eq!(result, vec!["line1", "line2", "line3"]);
    }
    
    #[tokio::test]
    async fn test_remove_duplicates_with_whitespace() {
        let input = "  line1  \nline2\n  line1\nline3  ";
        let result = remove_duplicates(input);
        assert_eq!(result, vec!["line1", "line2", "line3"]);
    }
    
    #[tokio::test]
    async fn test_remove_duplicates_empty_input() {
        let input = "";
        let result = remove_duplicates(input);
        assert!(result.is_empty());
    }
    
    // File validation tests
    
    #[tokio::test]
    async fn test_is_valid_file_type() {
        // Helper function to test file type validation
        fn is_valid_file_type(filename: &str) -> bool {
            filename.ends_with(".txt")
        }
        
        assert!(is_valid_file_type("test.txt"));
        assert!(!is_valid_file_type("test.pdf"));
        assert!(!is_valid_file_type("test"));
        assert!(!is_valid_file_type(""));
    }
    
    #[tokio::test]
    async fn test_is_file_size_valid() {
        // Helper function to test file size validation
        fn is_file_size_valid(content_size: usize) -> bool {
            content_size <= MAX_FILE_SIZE
        }
        
        assert!(is_file_size_valid(0));
        assert!(is_file_size_valid(1024));
        assert!(is_file_size_valid(MAX_FILE_SIZE));
        assert!(!is_file_size_valid(MAX_FILE_SIZE + 1));
    }
    
    // UTF-8 validation and edge case tests
    
    #[tokio::test]
    async fn test_utf8_validation() {
        // Valid UTF-8 bytes
        let valid_utf8 = vec![72, 101, 108, 108, 111]; // "Hello" in UTF-8
        let result = String::from_utf8_lossy(&valid_utf8);
        assert_eq!(result, "Hello");
        
        // Invalid UTF-8 bytes (the value 255 is not valid UTF-8)
        let invalid_utf8 = vec![72, 101, 108, 108, 111, 255];
        let result = String::from_utf8_lossy(&invalid_utf8);
        assert_eq!(result, "Hello�"); // Replacement character for invalid UTF-8
    }
    
    #[tokio::test]
    async fn test_edge_cases() {
        // Test with a string containing only whitespace
        let input = "   \n  \t  ";
        let result = remove_duplicates(input);
        assert!(result.is_empty());
        
        // Test with a string containing special characters
        let input = "line1\nline2\nline1\nline3\n!@#$%^&*()";
        let result = remove_duplicates(input);
        assert_eq!(result, vec!["line1", "line2", "line3", "!@#$%^&*()"]);
    }
    
    // Note: Testing the HTTP handlers directly is challenging in this setup
    // because they depend on external resources (files, templates) and
    // complex types (Multipart, HttpResponse). In a real-world scenario,
    // we would use integration tests for these handlers.
}
