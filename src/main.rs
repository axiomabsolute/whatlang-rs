use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::{Parser, Subcommand};
use whatlang::detect;

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Detect {
        /// The text to classify
        text: String,
    },
    #[command(arg_required_else_help = true)]
    Serve {
        /// Path to the SQLite DB to save stats to
        path: String,
        /// The port to host the service on
        port: u16,
    },
}

#[derive(Debug, Parser)]
#[command(name = "watl")]
#[command(about = "A tool for identifying natural languages", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[post("/detect")]
async fn detect_endpoint(req_body: String) -> impl Responder {
    detect(&req_body)
        .map(|info| HttpResponse::Ok().json(info))
        .ok_or(HttpResponse::BadRequest().body("Unable to detect language for body"))
        .map_or_else(|v| v, |v| v)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Detect { text } => {
            let info = detect(&text).expect("Should be able to detect.");
            let serialized =
                serde_json::to_string(&info).expect("Should be able to serialie result as JSON.");
            println!("{}", serialized);
            Ok(())
        }
        Commands::Serve { path, port } => {
            println!("Serve!! :{}, save to {}", port, path);
            HttpServer::new(|| App::new().service(health).service(detect_endpoint))
                .bind(("127.0.0.1", port))?
                .run()
                .await
        }
    }
}
