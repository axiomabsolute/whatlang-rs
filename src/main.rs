use actix_web::{
    get, guard, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
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

async fn index_graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://127.0.0.1:8080/gql")
                .finish(),
        )
}

async fn index_graphql(
    schema: web::Data<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

#[derive(Clone, SimpleObject)]
struct DetectResult {
    lang: String,
    script: String,
    confidence: f64,
}

struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn detect(&self, text: String) -> DetectResult {
        let info = detect(&text).expect("Should be able to detect");
        DetectResult {
            lang: info.lang().eng_name().to_owned(),
            script: info.script().name().to_owned(),
            confidence: info.confidence(),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
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
            println!("Serve :{}, stats to {}", port, path);
            HttpServer::new(|| {
                let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
                App::new()
                    .app_data(Data::new(schema.clone()))
                    .service(health)
                    .service(detect_endpoint)
                    .service(web::resource("/gql").guard(guard::Post()).to(index_graphql))
                    .service(web::resource("/gql").guard(guard::Get()).to(index_graphiql))
            })
            .bind(("127.0.0.1", port))?
            .run()
            .await
        }
    }
}
