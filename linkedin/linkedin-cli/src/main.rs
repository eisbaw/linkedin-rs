use std::fs;
use std::process;

use clap::{Parser, Subcommand};
use linkedin_api::auth::Session;
use linkedin_api::client::LinkedInClient;

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
        #[command(subcommand)]
        action: ProfileAction,
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
    /// Log in by providing a li_at cookie value
    Login {
        /// li_at cookie value from browser dev tools.
        /// Also accepts LINKEDIN_LI_AT environment variable.
        #[arg(long = "li-at")]
        li_at: Option<String>,
    },
    /// Check session status by calling the LinkedIn API
    Status {
        /// Only check locally (do not make an API call)
        #[arg(long)]
        local: bool,
    },
    /// Log out and clear stored session
    Logout,
}

#[derive(Subcommand)]
enum ProfileAction {
    /// Fetch the authenticated user's own profile
    Me {
        /// Output raw JSON instead of human-readable format
        #[arg(long)]
        json: bool,
    },
    /// View a profile by URN or vanity name (not yet implemented)
    View {
        /// LinkedIn profile URN or vanity name
        id: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { action } => match action {
            AuthAction::Login { li_at } => {
                if let Err(e) = cmd_auth_login(li_at).await {
                    eprintln!("error: {e}");
                    process::exit(1);
                }
            }
            AuthAction::Status { local } => {
                if let Err(e) = cmd_auth_status(local).await {
                    eprintln!("error: {e}");
                    process::exit(1);
                }
            }
            AuthAction::Logout => {
                if let Err(e) = cmd_auth_logout() {
                    eprintln!("error: {e}");
                    process::exit(1);
                }
            }
        },
        Commands::Profile { action } => match action {
            ProfileAction::Me { json } => {
                if let Err(e) = cmd_profile_me(json).await {
                    eprintln!("error: {e}");
                    process::exit(1);
                }
            }
            ProfileAction::View { id } => {
                println!("Profile view for '{}': not yet implemented", id);
            }
        },
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

/// Handle `auth login --li-at <value>`.
///
/// Resolves the li_at value from the CLI flag or the `LINKEDIN_LI_AT`
/// environment variable. Generates a fresh JSESSIONID, creates a Session,
/// and saves it to the default path.
async fn cmd_auth_login(li_at_flag: Option<String>) -> Result<(), String> {
    let li_at = li_at_flag
        .or_else(|| std::env::var("LINKEDIN_LI_AT").ok())
        .ok_or_else(|| {
            "li_at cookie value required: use --li-at <value> or set LINKEDIN_LI_AT env var"
                .to_string()
        })?;

    if li_at.trim().is_empty() {
        return Err("li_at cookie value must not be empty".to_string());
    }

    // Generate a fresh JSESSIONID for this session.
    let client = LinkedInClient::new().map_err(|e| format!("failed to create client: {e}"))?;
    let jsessionid = client.jsessionid().to_string();

    let session = Session::new(li_at, jsessionid);
    let path = Session::default_path().map_err(|e| format!("{e}"))?;
    session.save(&path).map_err(|e| format!("{e}"))?;

    println!("Session saved to {}", path.display());
    println!("JSESSIONID: {}...", &session.jsessionid[..10]);
    println!(
        "li_at: {}...",
        &session.li_at[..session.li_at.len().min(10)]
    );
    Ok(())
}

/// Handle `auth status [--local]`.
///
/// Without `--local`, loads the session and calls GET /voyager/api/me to verify
/// the session is still valid server-side. With `--local`, only checks the
/// session file on disk (no network request).
async fn cmd_auth_status(local_only: bool) -> Result<(), String> {
    let path = Session::default_path().map_err(|e| format!("{e}"))?;

    if !path.exists() {
        println!("No session found at {}", path.display());
        println!("Status: not logged in");
        return Ok(());
    }

    let session = Session::load(&path).map_err(|e| format!("{e}"))?;

    println!("Session file: {}", path.display());
    println!("Created at: {}", session.created_at);
    println!(
        "JSESSIONID: {}...",
        &session.jsessionid[..session.jsessionid.len().min(10)]
    );
    println!(
        "li_at: {}...",
        &session.li_at[..session.li_at.len().min(10)]
    );

    if !session.is_valid() {
        println!("Status: invalid (empty li_at cookie)");
        return Ok(());
    }

    if local_only {
        println!("Status: valid (local check only -- session may be expired server-side)");
        return Ok(());
    }

    // Hit the live API to verify the session is actually valid.
    println!("Checking session against LinkedIn API...");
    let client =
        LinkedInClient::with_session(&session).map_err(|e| format!("client error: {e}"))?;

    match client.get_me().await {
        Ok(me) => {
            println!("Status: authenticated");
            // Try to extract a display name from the response.
            if let Some(mini) = me.get("miniProfile") {
                let first = mini.get("firstName").and_then(|v| v.as_str()).unwrap_or("");
                let last = mini.get("lastName").and_then(|v| v.as_str()).unwrap_or("");
                if !first.is_empty() || !last.is_empty() {
                    println!("Logged in as: {} {}", first, last);
                }
            }
            if let Some(id) = me.get("plainId").and_then(|v| v.as_i64()) {
                println!("Member ID: {}", id);
            }
            Ok(())
        }
        Err(e) => {
            println!("Status: session invalid or expired");
            println!("API error: {e}");
            Ok(())
        }
    }
}

/// Handle `auth logout`.
///
/// Deletes the session file from disk.
fn cmd_auth_logout() -> Result<(), String> {
    let path = Session::default_path().map_err(|e| format!("{e}"))?;

    if !path.exists() {
        println!("No session file found at {}", path.display());
        return Ok(());
    }

    fs::remove_file(&path).map_err(|e| format!("failed to remove session file: {e}"))?;
    println!("Session removed: {}", path.display());
    Ok(())
}

/// Handle `profile me [--json]`.
///
/// Loads the session, creates a client, calls GET /voyager/api/me, and
/// prints the result. With `--json`, outputs raw pretty-printed JSON.
/// Without `--json`, outputs a human-readable summary.
async fn cmd_profile_me(raw_json: bool) -> Result<(), String> {
    let path = Session::default_path().map_err(|e| format!("{e}"))?;

    if !path.exists() {
        return Err(format!(
            "no session found at {} -- run `auth login` first",
            path.display()
        ));
    }

    let session = Session::load(&path).map_err(|e| format!("{e}"))?;

    if !session.is_valid() {
        return Err("session is invalid (empty li_at cookie)".to_string());
    }

    let client =
        LinkedInClient::with_session(&session).map_err(|e| format!("client error: {e}"))?;

    let me = client
        .get_me()
        .await
        .map_err(|e| format!("API call failed: {e}"))?;

    if raw_json {
        let pretty =
            serde_json::to_string_pretty(&me).map_err(|e| format!("JSON format error: {e}"))?;
        println!("{}", pretty);
    } else {
        print_me_summary(&me);
    }

    Ok(())
}

/// Print a human-readable summary of the /voyager/api/me response.
///
/// Extracts known fields from the response and prints them. The exact
/// response structure depends on LinkedIn's API version, so this is
/// best-effort. Unknown fields are skipped rather than causing errors.
fn print_me_summary(me: &serde_json::Value) {
    if let Some(mini) = me.get("miniProfile") {
        let first = mini.get("firstName").and_then(|v| v.as_str()).unwrap_or("");
        let last = mini.get("lastName").and_then(|v| v.as_str()).unwrap_or("");
        if !first.is_empty() || !last.is_empty() {
            println!("Name: {} {}", first, last);
        }

        if let Some(occ) = mini.get("occupation").and_then(|v| v.as_str()) {
            println!("Headline: {}", occ);
        }

        if let Some(urn) = mini.get("entityUrn").and_then(|v| v.as_str()) {
            println!("URN: {}", urn);
        }

        if let Some(vanity) = mini.get("publicIdentifier").and_then(|v| v.as_str()) {
            println!("Public ID: {}", vanity);
        }
    }

    if let Some(id) = me.get("plainId").and_then(|v| v.as_i64()) {
        println!("Member ID: {}", id);
    }

    if let Some(premium) = me.get("premiumSubscriber").and_then(|v| v.as_bool()) {
        println!("Premium: {}", if premium { "yes" } else { "no" });
    }

    // Print top-level keys for discoverability.
    if let Some(obj) = me.as_object() {
        let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        if !keys.is_empty() {
            println!("Response keys: {}", keys.join(", "));
        }
    }
}
