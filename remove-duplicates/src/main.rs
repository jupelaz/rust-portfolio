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
            return HttpResponse::BadRequest().body("Solo se permiten archivos .txt");
        }

        let mut content = Vec::new();

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(_) => return HttpResponse::BadRequest().body("Error leyendo archivo"),
            };
            if content.len() + data.len() > MAX_FILE_SIZE {
                return HttpResponse::BadRequest().body("Archivo demasiado grande");
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

    HttpResponse::BadRequest().body("No se recibió archivo")
}

async fn download(form: web::Form<std::collections::HashMap<String, String>>) -> impl Responder {
    if let Some(content) = form.get("content") {
        return HttpResponse::Ok()
            .append_header(("Content-Disposition", "attachment; filename=\"cleaned.txt\""))
            .body(content.clone());
    }

    HttpResponse::BadRequest().body("No se pudo descargar")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Servidor en http://0.0.0.0:8080");

    // Motor de plantillas Minijinja
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
