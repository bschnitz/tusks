use tusks::tusks;

#[tusks(root)]
#[command(about = "Test default functions for modules", version = "1.0.0")]
pub mod cli {
    /// Root command
    pub fn root() {
        println!("Root command executed");
    }

    /// Module with default function and parameters
    #[command(about = "Git operations")]
    pub mod git {
        pub struct Parameters<'a> {
            #[arg(long, help = "Repository path")]
            pub repo: &'a Option<String>,
        }

        /// Default action - show status
        #[default]
        pub fn status(params: &Parameters) {
            println!("Git status");
            if let Some(repo) = params.repo {
                println!("Repository: {}", repo);
            }
        }

        /// Commit changes
        pub fn commit(
            params: &Parameters,
            #[arg(help = "Commit message")]
            message: String,
        ) {
            println!("Committing: {}", message);
            if let Some(repo) = params.repo {
                println!("Repository: {}", repo);
            }
        }

        /// Push changes
        pub fn push(params: &Parameters) {
            println!("Pushing changes");
            if let Some(repo) = params.repo {
                println!("Repository: {}", repo);
            }
        }
    }

    /// Module with default function, no parameters
    #[command(about = "Build operations")]
    pub mod build {
        /// Default action - build project
        #[default]
        pub fn default() {
            println!("Building project with default settings");
        }

        /// Clean build artifacts
        pub fn clean() {
            println!("Cleaning build artifacts");
        }

        /// Run tests
        pub fn test() {
            println!("Running tests");
        }
    }

    /// Module with default and external subcommands
    #[command(about = "Task runner", allow_external_subcommands = true)]
    pub mod run {
        pub struct Parameters<'a> {
            #[arg(long, help = "Working directory")]
            pub workdir: &'a Option<String>,
        }

        /// Default action - execute external command
        #[default]
        pub fn execute(params: &Parameters, args: Vec<String>) {
            if args.is_empty() {
                println!("No command specified");
            } else {
                println!("Running external command: {}", args[0]);
                if args.len() > 1 {
                    println!("Arguments: {:?}", &args[1..]);
                }
            }
            
            if let Some(dir) = params.workdir {
                println!("Working directory: {}", dir);
            }
        }

        /// Built-in task
        pub fn builtin(
            #[arg(help = "Task name")]
            task: String,
        ) {
            println!("Running built-in task: {}", task);
        }
    }

    /// Nested modules with defaults
    #[command(about = "Server operations")]
    pub mod server {
        pub struct Parameters<'a> {
            #[arg(long, help = "Server port")]
            pub port: &'a Option<u16>,
        }

        /// Default - show server status
        #[default]
        pub fn status(params: &Parameters) {
            let port = params.port.unwrap_or(8080);
            println!("Server status on port: {}", port);
        }

        /// Start server
        pub fn start(params: &Parameters) {
            let port = params.port.unwrap_or(8080);
            println!("Starting server on port: {}", port);
        }

        /// Config submodule with its own default
        #[command(about = "Server configuration")]
        pub mod config {
            /// Default - show config
            #[default]
            pub fn show() {
                println!("Showing server configuration");
            }

            /// Update config
            pub fn update(
                #[arg(help = "Key")]
                key: String,
                #[arg(help = "Value")]
                value: String,
            ) {
                println!("Updating config: {}={}", key, value);
            }
        }
    }

    /// Module without default function
    #[command(about = "Database operations")]
    pub mod database {
        pub struct Parameters<'a> {
            #[arg(long, help = "Connection string")]
            pub connection: &'a String,
        }

        /// Connect to database
        pub fn connect(params: &Parameters) {
            println!("Connecting to: {}", params.connection);
        }

        /// Query database
        pub fn query(
            params: &Parameters,
            #[arg(help = "SQL query")]
            sql: String,
        ) {
            println!("Executing: {}", sql);
            println!("Connection: {}", params.connection);
        }
    }

    /// Module with default returning exit code
    #[command(about = "Health checks")]
    pub mod health {
        /// Default - check all systems
        #[default]
        pub fn check() -> u8 {
            println!("Running health check");
            
            println!("✓ Health check passed");
            0
        }

        /// Check specific component
        pub fn component(
            #[arg(help = "Component name")]
            name: String,
        ) {
            println!("Checking component: {}", name);
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
