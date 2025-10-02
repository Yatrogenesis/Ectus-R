// AION-R Enterprise Platform CLI
// Command-line interface for interacting with AION-R

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod auth;
mod client;
mod commands;
mod config;
mod output;
mod utils;

use client::AionClient;
use config::CliConfig;

/// Ectus-R Autonomous Software Engineer CLI
#[derive(Parser)]
#[command(name = "ectus-r")]
#[command(about = "Ectus-R - The Autonomous Software Engineer. From business logic to production code in minutes.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Yatrogenesis Team")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// API endpoint URL
    #[arg(short, long, global = true, env = "AION_API_URL")]
    api_url: Option<String>,

    /// Output format (json, yaml, table, plain)
    #[arg(short, long, global = true, default_value = "table")]
    output: String,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Quiet mode (suppress non-essential output)
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive autonomous project creation
    New {
        /// Project name (optional, will prompt if not provided)
        #[arg(short, long)]
        name: Option<String>,
        /// Skip interactive prompts and use defaults
        #[arg(short, long)]
        quick: bool,
    },
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    /// Code generation commands
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    },
    /// Requirements analysis commands
    Requirements {
        #[command(subcommand)]
        command: RequirementsCommands,
    },
    /// AI processing commands
    AI {
        #[command(subcommand)]
        command: AICommands,
    },
    /// Project management commands
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    /// Configuration commands
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Status and information commands
    Status {
        #[command(subcommand)]
        command: StatusCommands,
    },
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login to AION-R platform
    Login {
        /// Email address
        #[arg(short, long)]
        email: Option<String>,
        /// Password (will prompt if not provided)
        #[arg(short, long)]
        password: Option<String>,
        /// MFA code
        #[arg(short, long)]
        mfa_code: Option<String>,
    },
    /// Logout from AION-R platform
    Logout,
    /// Show current authentication status
    Status,
    /// Register new user account
    Register {
        /// Email address
        #[arg(short, long)]
        email: String,
        /// First name
        #[arg(short, long)]
        first_name: String,
        /// Last name
        #[arg(short, long)]
        last_name: String,
        /// Company name
        #[arg(short, long)]
        company: Option<String>,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    /// Generate code from requirements
    Code {
        /// Requirements description
        #[arg(short, long)]
        requirements: Option<String>,
        /// Requirements file path
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Programming language
        #[arg(short, long, default_value = "rust")]
        language: String,
        /// Framework to use
        #[arg(short = 'F', long)]
        framework: Option<String>,
        /// Architecture pattern
        #[arg(short, long, default_value = "layered")]
        architecture: String,
        /// Optimization level
        #[arg(short, long, default_value = "balanced")]
        optimization: String,
        /// Output directory
        #[arg(short, long, default_value = "./generated")]
        output_dir: PathBuf,
        /// Include tests
        #[arg(long, default_value = "true")]
        include_tests: bool,
        /// Include documentation
        #[arg(long, default_value = "true")]
        include_docs: bool,
    },
    /// List previous generations
    List {
        /// Number of results per page
        #[arg(short, long, default_value = "10")]
        limit: u32,
        /// Page number
        #[arg(short, long, default_value = "1")]
        page: u32,
    },
    /// Get details of a specific generation
    Get {
        /// Generation ID
        id: String,
    },
    /// Download generated code
    Download {
        /// Generation ID
        id: String,
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// Delete a generation
    Delete {
        /// Generation ID
        id: String,
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum RequirementsCommands {
    /// Analyze requirements
    Analyze {
        /// Requirements text
        #[arg(short, long)]
        requirements: Option<String>,
        /// Requirements file
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Output detailed analysis
        #[arg(short, long)]
        detailed: bool,
    },
    /// Optimize requirements
    Optimize {
        /// Requirements text
        #[arg(short, long)]
        requirements: Option<String>,
        /// Requirements file
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Validate requirements
    Validate {
        /// Requirements text
        #[arg(short, long)]
        requirements: Option<String>,
        /// Requirements file
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum AICommands {
    /// Text processing commands
    Text {
        #[command(subcommand)]
        command: TextCommands,
    },
    /// Image processing commands
    Vision {
        #[command(subcommand)]
        command: VisionCommands,
    },
    /// Audio processing commands
    Audio {
        #[command(subcommand)]
        command: AudioCommands,
    },
    /// List available models
    Models,
}

#[derive(Subcommand)]
enum TextCommands {
    /// Analyze text
    Analyze {
        /// Text to analyze
        #[arg(short, long)]
        text: Option<String>,
        /// Text file path
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Analysis types (sentiment, entities, language)
        #[arg(short = 'A', long, value_delimiter = ',')]
        analysis_types: Vec<String>,
    },
    /// Generate text
    Generate {
        /// Prompt for generation
        #[arg(short, long)]
        prompt: String,
        /// Maximum tokens
        #[arg(short, long, default_value = "200")]
        max_tokens: u32,
        /// Temperature (0.0-1.0)
        #[arg(short, long, default_value = "0.7")]
        temperature: f32,
    },
    /// Summarize text
    Summarize {
        /// Text to summarize
        #[arg(short, long)]
        text: Option<String>,
        /// Text file path
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Summary length
        #[arg(short, long, default_value = "medium")]
        length: String,
    },
    /// Translate text
    Translate {
        /// Text to translate
        #[arg(short, long)]
        text: Option<String>,
        /// Text file path
        #[arg(short, long)]
        file: Option<PathBuf>,
        /// Target language
        #[arg(short = 'l', long)]
        target_language: String,
        /// Source language (auto-detect if not specified)
        #[arg(short = 's', long)]
        source_language: Option<String>,
    },
}

#[derive(Subcommand)]
enum VisionCommands {
    /// Analyze image
    Analyze {
        /// Image file path
        file: PathBuf,
        /// Analysis types (objects, faces, text, scene)
        #[arg(short, long, value_delimiter = ',')]
        analysis_types: Vec<String>,
    },
    /// Classify image
    Classify {
        /// Image file path
        file: PathBuf,
        /// Number of top predictions
        #[arg(short, long, default_value = "5")]
        top_k: u32,
    },
    /// Detect objects in image
    DetectObjects {
        /// Image file path
        file: PathBuf,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.5")]
        confidence: f32,
    },
}

#[derive(Subcommand)]
enum AudioCommands {
    /// Transcribe audio
    Transcribe {
        /// Audio file path
        file: PathBuf,
        /// Language hint
        #[arg(short, long)]
        language: Option<String>,
        /// Include timestamps
        #[arg(short, long)]
        timestamps: bool,
        /// Enable speaker detection
        #[arg(short, long)]
        speakers: bool,
    },
    /// Analyze audio
    Analyze {
        /// Audio file path
        file: PathBuf,
        /// Analysis types (emotion, sentiment, quality)
        #[arg(short, long, value_delimiter = ',')]
        analysis_types: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// List projects
    List,
    /// Create new project
    Create {
        /// Project name
        name: String,
        /// Project description
        #[arg(short, long)]
        description: Option<String>,
        /// Project template
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Get project details
    Get {
        /// Project ID or name
        id: String,
    },
    /// Update project
    Update {
        /// Project ID or name
        id: String,
        /// New name
        #[arg(short, long)]
        name: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete project
    Delete {
        /// Project ID or name
        id: String,
        /// Force deletion
        #[arg(short, long)]
        force: bool,
    },
    /// Deploy project
    Deploy {
        /// Project ID or name
        id: String,
        /// Environment
        #[arg(short, long, default_value = "staging")]
        environment: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Reset configuration to defaults
    Reset {
        /// Force reset without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum StatusCommands {
    /// Show platform status
    Platform,
    /// Show user account information
    Account,
    /// Show usage statistics
    Usage {
        /// Time period (day, week, month, year)
        #[arg(short, long, default_value = "month")]
        period: String,
    },
    /// Show billing information
    Billing,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize configuration
    let config = CliConfig::load(cli.config.as_deref())?;

    // Create API client
    let client = AionClient::new(
        cli.api_url.unwrap_or_else(|| config.api_url.clone()),
        config.clone(),
    )?;

    // Set up output formatter
    let output_format = output::OutputFormat::from_str(&cli.output)?;

    // Execute command
    match cli.command {
        Commands::New { name, quick } => {
            commands::new::handle_new_command(name, quick, &client, &output_format).await?;
        }
        Commands::Auth { command } => {
            commands::auth::handle_auth_command(command, &client, &output_format).await?;
        }
        Commands::Generate { command } => {
            commands::generate::handle_generate_command(command, &client, &output_format).await?;
        }
        Commands::Requirements { command } => {
            commands::requirements::handle_requirements_command(command, &client, &output_format).await?;
        }
        Commands::AI { command } => {
            commands::ai::handle_ai_command(command, &client, &output_format).await?;
        }
        Commands::Project { command } => {
            commands::project::handle_project_command(command, &client, &output_format).await?;
        }
        Commands::Config { command } => {
            commands::config::handle_config_command(command, &config, &output_format).await?;
        }
        Commands::Status { command } => {
            commands::status::handle_status_command(command, &client, &output_format).await?;
        }
    }

    Ok(())
}