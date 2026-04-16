use tusks::tusks;

mod database;

#[tusks(root, tasks)]
#[command(about = "Task management system", version = "1.0.0")]
pub mod tasks {
    /// Root level task
    pub fn hello(
        #[arg(help = "Name to greet")]
        name: String,
    ) {
        println!("Hello, {}!", name);
    }

    /// Git operations
    #[command(about = "Git version control")]
    pub mod git {
        #[command(about = "Clone a repository")]
        pub fn clone(
            #[arg(help = "Repository URL")]
            url: String,
        ) {
            println!("Cloning: {}", url);
        }

        /// Commit changes
        pub fn commit(
            #[arg(help = "Commit message")]
            message: String,
        ) {
            println!("Committing: {}", message);
        }

        /// Push changes
        pub fn push() {
            println!("Pushing changes");
        }

        /// Advanced git operations
        #[command(about = "Advanced git commands")]
        pub mod advanced {
            /// Rebase branch
            pub fn rebase(
                #[arg(help = "Base branch")]
                base: String,
            ) {
                println!("Rebasing onto: {}", base);
            }

            /// Cherry-pick commit
            pub fn cherry_pick(
                #[arg(help = "Commit hash")]
                commit: String,
            ) {
                println!("Cherry-picking: {}", commit);
            }
        }
    }

    /// Docker operations
    #[command(about = "Docker container management")]
    pub mod docker {
        /// Build image
        pub fn build(
            #[arg(help = "Build context")]
            context: String,
        ) {
            println!("Building from: {}", context);
        }

        /// Run container
        pub fn run(
            #[arg(help = "Image name")]
            image: String,
        ) {
            println!("Running: {}", image);
        }

        /// Stop container
        pub fn stop(
            #[arg(help = "Container ID")]
            container: String,
        ) {
            println!("Stopping: {}", container);
        }

        /// Container operations
        #[command(about = "Container management")]
        pub mod container {
            /// List containers
            pub fn list() {
                println!("Listing containers");
            }

            /// Inspect container
            pub fn inspect(
                #[arg(help = "Container ID")]
                id: String,
            ) {
                println!("Inspecting: {}", id);
            }
        }

        /// Image operations
        #[command(about = "Image management")]
        pub mod image {
            /// List images
            pub fn list() {
                println!("Listing images");
            }

            /// Remove image
            pub fn remove(
                #[arg(help = "Image ID")]
                id: String,
            ) {
                println!("Removing: {}", id);
            }

            /// Pull image
            pub fn pull(
                #[arg(help = "Image name")]
                name: String,
            ) {
                println!("Pulling: {}", name);
            }
        }
    }

    /// Database operations
    #[command(about = "Database management")]
    pub use crate::database::tasks as database;

    /// Test operations
    #[command(about = "Testing framework")]
    pub mod test {
        /// Run unit tests
        pub fn unit() {
            println!("Running unit tests");
        }

        /// Run integration tests
        pub fn integration() {
            println!("Running integration tests");
        }

        /// Run end-to-end tests
        pub fn e2e() {
            println!("Running e2e tests");
        }
    }

    /// Deployment operations
    #[command(about = "Deployment tasks")]
    pub mod deploy {
        /// Deploy to staging
        pub fn staging(
            #[arg(help = "Version to deploy")]
            version: String,
        ) {
            println!("Deploying {} to staging", version);
        }

        /// Deploy to production
        pub fn production(
            #[arg(help = "Version to deploy")]
            version: String,
        ) {
            println!("Deploying {} to production", version);
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(tasks::exec_cli().unwrap_or(0))
}
