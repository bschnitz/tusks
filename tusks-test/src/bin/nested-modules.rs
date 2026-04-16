use tusks::tusks;

#[tusks(root)]
#[command(about = "Test nested module structures", version = "1.0.0")]
pub mod cli {
    /// Root level command
    pub fn root_command(
        #[arg(help = "Message to display")]
        message: String,
    ) {
        println!("Root: {}", message);
    }

    /// Level 1 module
    #[command(about = "First level operations")]
    pub mod level1 {
        /// Command in level 1
        pub fn command(
            #[arg(help = "Value")]
            value: String,
        ) {
            println!("Level1 command: {}", value);
        }

        /// Another command in level 1
        pub fn another(
            #[arg(long, help = "Enable feature")]
            feature: bool,
        ) {
            println!("Level1 another: feature={}", feature);
        }

        /// Level 2 module
        #[command(about = "Second level operations")]
        pub mod level2 {
            /// Command in level 2
            pub fn command(
                #[arg(help = "Input")]
                input: String,
            ) {
                println!("Level2 command: {}", input);
            }

            /// Level 3 module
            #[command(about = "Third level operations")]
            pub mod level3 {
                /// Command in level 3
                pub fn command(
                    #[arg(help = "Data")]
                    data: String,
                ) {
                    println!("Level3 command: {}", data);
                }

                /// Deep command
                pub fn deep(
                    #[arg(long, help = "Count")]
                    count: Option<u32>,
                ) {
                    println!("Level3 deep: count={:?}", count);
                }
            }
        }
    }

    /// Database operations module
    #[command(about = "Database management")]
    pub mod database {
        /// Connect to database
        pub fn connect(
            #[arg(long, help = "Connection string")]
            connection: String,
        ) {
            println!("Connecting to: {}", connection);
        }

        /// Query operations
        #[command(about = "Query operations")]
        pub mod query {
            /// Execute a query
            pub fn execute(
                #[arg(help = "SQL query")]
                sql: String,
            ) {
                println!("Executing query: {}", sql);
            }

            /// List all queries
            pub fn list() {
                println!("Listing all queries");
            }
        }

        /// Migration operations
        #[command(about = "Database migrations")]
        pub mod migrate {
            /// Run migrations
            pub fn up(
                #[arg(long, help = "Target version")]
                version: Option<String>,
            ) {
                println!("Running migrations up to: {:?}", version);
            }

            /// Rollback migrations
            pub fn down(
                #[arg(long, help = "Number of steps")]
                steps: Option<u32>,
            ) {
                println!("Rolling back {} steps", steps.unwrap_or(1));
            }
        }
    }

    /// Server operations
    #[command(about = "Server management")]
    pub mod server {
        /// Start the server
        pub fn start(
            #[arg(long, help = "Port number")]
            port: Option<u16>,
        ) {
            println!("Starting server on port: {}", port.unwrap_or(8080));
        }

        /// Stop the server
        pub fn stop() {
            println!("Stopping server");
        }

        /// Configuration module
        #[command(about = "Server configuration")]
        pub mod config {
            /// Show configuration
            pub fn show() {
                println!("Showing configuration");
            }

            /// Update configuration
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
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
