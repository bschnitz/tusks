use tusks::tusks;

#[tusks(root)]
#[command(about = "Async CLI test", version = "1.0.0")]
pub mod cli {
    /// Async command returning nothing
    pub async fn fetch(url: String) {
        println!("Fetching: {}", url);
    }

    /// Sync command in the same module
    pub fn greet(name: String) {
        println!("Hello, {}!", name);
    }

    /// Async command returning u8
    pub async fn check() -> u8 {
        println!("Running async check...");
        0
    }

    /// Async command returning Option<u8>
    pub async fn validate(file: String) -> Option<u8> {
        println!("Validating: {}", file);
        Some(0)
    }

    /// Async submodule
    #[command(about = "Database operations")]
    pub mod db {
        /// Async nested command
        pub async fn migrate(version: String) {
            println!("Migrating to: {}", version);
        }

        /// Sync nested command
        pub fn status() {
            println!("DB status: ok");
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
