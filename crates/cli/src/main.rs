use clap::{command, Parser, Subcommand};
use whatlang::detect;

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Detect {
        /// The text to classify
        text: String,
    },
}

#[derive(Debug, Parser)]
#[command(name = "watl")]
#[command(about = "A tool for identifying natural languages", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    }
}
