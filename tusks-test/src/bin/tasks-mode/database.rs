use tusks::tusks;

#[tusks()]
pub mod tasks {
    // Reference to parent module
    pub use crate::tasks as parent_;

    /// Create database
    pub fn create(
        #[arg(help = "Database name")]
        name: String,
    ) {
        println!("Creating database: {}", name);
    }

    /// Drop database
    pub fn drop(
        #[arg(help = "Database name")]
        name: String,
    ) {
        println!("Dropping database: {}", name);
    }

    /// Migration tasks
    #[command(about = "Database migrations")]
    pub mod migrate {
        /// Run migrations
        pub fn up() {
            println!("Running migrations up");
        }

        /// Rollback migrations
        pub fn down() {
            println!("Rolling back migrations");
        }

        /// Show migration status
        pub fn status() {
            println!("Migration status");
        }
    }
}
