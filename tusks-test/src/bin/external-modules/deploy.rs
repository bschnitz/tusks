use tusks::tusks;

#[tusks()]
pub mod cli {
    // Reference to parent module
    pub use crate::cli as parent_;

    pub struct Parameters<'a> {
        #[arg(long, help = "Target environment")]
        pub environment: &'a String,
    }

    /// Deploy application
    pub fn start(
        params: &Parameters,
        #[arg(help = "Version to deploy")]
        version: String,
    ) {
        println!("Deploying version: {}", version);
        println!("Environment: {}", params.environment);
        
        if *params.super_.verbose {
            println!("Verbose: detailed deployment");
        }
        if let Some(dir) = params.super_.workdir {
            println!("Working directory: {}", dir);
        }
    }

    /// Rollback deployment
    pub fn rollback(
        params: &Parameters,
        #[arg(long, help = "Number of versions to rollback")]
        steps: Option<u32>,
    ) {
        let steps = steps.unwrap_or(1);
        println!("Rolling back {} version(s)", steps);
        println!("Environment: {}", params.environment);
    }

    /// Check deployment status
    pub fn status(params: &Parameters) {
        println!("Deployment status for: {}", params.environment);
        
        if *params.super_.verbose {
            println!("Verbose: detailed status");
        }
    }
}
