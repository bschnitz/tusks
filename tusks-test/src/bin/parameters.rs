use tusks::tusks;

#[tusks(root)]
#[command(about = "Test parameter chaining across modules", version = "1.0.0")]
pub mod cli {
    // Root-level parameters
    pub struct Parameters<'a> {
        #[arg(long, help = "Enable verbose output")]
        pub verbose: &'a bool,
        
        #[arg(long, help = "Configuration file")]
        pub config: &'a Option<String>,
    }

    /// Root command using root parameters
    pub fn status(params: &Parameters) {
        println!("Status check");
        if *params.verbose {
            println!("Verbose: enabled");
        }
        if let Some(cfg) = params.config {
            println!("Config: {}", cfg);
        }
    }

    /// Database module with its own parameters
    #[command(about = "Database operations")]
    pub mod database {
        pub struct Parameters<'a> {
            #[arg(long, help = "Database connection string")]
            pub connection: &'a String,
            
            #[arg(long, help = "Database timeout")]
            pub timeout: &'a Option<u32>,
        }

        /// Connect using database and root parameters
        pub fn connect(params: &Parameters) {
            println!("Connecting to: {}", params.connection);
            
            if let Some(timeout) = params.timeout {
                println!("Timeout: {}s", timeout);
            }
            
            // Access parent (root) parameters via super_
            if *params.super_.verbose {
                println!("Verbose mode enabled");
            }
            if let Some(cfg) = params.super_.config {
                println!("Using config: {}", cfg);
            }
        }

        /// Query with additional arguments
        pub fn query(
            params: &Parameters,
            #[arg(help = "SQL query to execute")]
            sql: String,
        ) {
            println!("Executing: {}", sql);
            println!("Connection: {}", params.connection);
            
            if *params.super_.verbose {
                println!("Query details: length={}", sql.len());
            }
        }

        /// Nested module with its own parameters
        #[command(about = "Backup operations")]
        pub mod backup {
            pub struct Parameters<'a> {
                #[arg(long, help = "Backup path")]
                pub path: &'a String,
                
                #[arg(long, help = "Compression level")]
                pub compression: &'a Option<u32>,
            }

            /// Create backup using all three parameter levels
            pub fn create(params: &Parameters) {
                println!("Creating backup to: {}", params.path);
                
                if let Some(level) = params.compression {
                    println!("Compression: {}", level);
                }
                
                // Access parent (database) parameters
                println!("Database: {}", params.super_.connection);
                
                // Access grandparent (root) parameters via super_.super_
                if *params.super_.super_.verbose {
                    println!("Verbose: showing detailed backup info");
                }
                if let Some(cfg) = params.super_.super_.config {
                    println!("Root config: {}", cfg);
                }
            }

            /// Restore with command argument
            pub fn restore(
                params: &Parameters,
                #[arg(help = "Backup file to restore")]
                backup_file: String,
            ) {
                println!("Restoring from: {}", backup_file);
                println!("Target path: {}", params.path);
                println!("Database: {}", params.super_.connection);
                
                if *params.super_.super_.verbose {
                    println!("Verbose restore mode");
                }
            }
        }
    }

    /// Server module without its own parameters
    #[command(about = "Server operations")]
    pub mod server {
        /// Start server using only root parameters
        pub fn start(
            params: &Parameters,
            #[arg(long, help = "Port number")]
            port: Option<u16>,
        ) {
            let port = port.unwrap_or(8080);
            println!("Starting server on port: {}", port);
            
            if *params.super_.verbose {
                println!("Verbose: detailed startup");
            }
            if let Some(cfg) = params.super_.config {
                println!("Config: {}", cfg);
            }
        }

        /// Nested module with parameters, parent has none
        #[command(about = "Deployment operations")]
        pub mod deploy {
            pub struct Parameters<'a> {
                #[arg(long, help = "Environment name")]
                pub environment: &'a String,
            }

            /// Deploy using deploy and root parameters
            pub fn run(
                params: &Parameters,
                #[arg(help = "Version to deploy")]
                version: String,
            ) {
                println!("Deploying version: {}", version);
                println!("Environment: {}", params.environment);
                
                // Access grandparent (root) parameters, skipping parent (server) which has none
                if *params.super_.super_.verbose {
                    println!("Verbose deployment");
                }
            }
        }
    }

    /// Module to test optional parameters usage
    #[command(about = "Test operations")]
    pub mod test {
        pub struct Parameters<'a> {
            #[arg(long, help = "Test suite")]
            pub suite: &'a Option<String>,
        }

        /// Command that may or may not use parameters
        pub fn run(params: &Parameters) {
            if let Some(suite) = params.suite {
                println!("Running test suite: {}", suite);
            } else {
                println!("Running default tests");
            }
            
            if *params.super_.verbose {
                println!("Verbose test output");
            }
        }

        /// Command without using params at all
        pub fn list() {
            println!("Listing all available tests");
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
