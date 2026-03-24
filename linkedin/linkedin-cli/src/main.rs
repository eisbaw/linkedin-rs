use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "linkedin-cli")]
#[command(about = "CLI for LinkedIn API", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with LinkedIn
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// View profiles
    Profile {
        /// LinkedIn profile URN or vanity name
        id: Option<String>,
    },
    /// Messaging operations
    Messages,
    /// Feed and posts
    Feed,
    /// Connection management
    Connections,
}

#[derive(Subcommand)]
enum AuthAction {
    /// Log in to LinkedIn
    Login,
    /// Show current session status
    Status,
    /// Log out and clear stored session
    Logout,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { action } => match action {
            AuthAction::Login => {
                println!("Auth login: not yet implemented");
            }
            AuthAction::Status => {
                println!("Auth status: not yet implemented");
            }
            AuthAction::Logout => {
                println!("Auth logout: not yet implemented");
            }
        },
        Commands::Profile { id } => {
            let target = id.as_deref().unwrap_or("me");
            println!("Profile view for '{}': not yet implemented", target);
        }
        Commands::Messages => {
            println!("Messages: not yet implemented");
        }
        Commands::Feed => {
            println!("Feed: not yet implemented");
        }
        Commands::Connections => {
            println!("Connections: not yet implemented");
        }
    }
}
