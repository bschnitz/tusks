use tusks::tusks;

// External modules
mod git;
mod docker;
mod deploy;

#[tusks(root)]
#[command(about = "Test external module integration", version = "1.0.0")]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long, help = "Enable verbose output")]
        pub verbose: &'a bool,
        
        #[arg(long, help = "Working directory")]
        pub workdir: &'a Option<String>,
    }

    /// Root level command
    pub fn status(params: &Parameters) {
        println!("Status check");
        if *params.verbose {
            println!("Verbose: enabled");
        }
        if let Some(dir) = params.workdir {
            println!("Working directory: {}", dir);
        }
    }

    /// Git operations (external module)
    #[command(about = "Git version control")]
    pub use crate::git::cli as git;

    /// Docker operations (external module)
    #[command(about = "Docker container management")]
    pub use crate::docker::cli as docker;

    /// Deployment operations (external module with custom name)
    #[command(about = "Deploy applications", name = "deploy")]
    pub use crate::deploy::cli as deployment;

    /// Local module for comparison
    #[command(about = "Local build operations")]
    pub mod build {
        /// Compile the project
        pub fn compile(
            #[arg(long, help = "Build target")]
            target: Option<String>,
        ) {
            println!("Compiling for target: {:?}", target.unwrap_or_else(|| "default".to_string()));
        }

        /// Clean build artifacts
        pub fn clean() {
            println!("Cleaning build artifacts");
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
