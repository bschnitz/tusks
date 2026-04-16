use tusks::tusks;

#[tusks]
pub mod cli {
    // Reference to parent module
    pub use crate::cli as parent_;

    pub struct Parameters<'a> {
        #[arg(long, help = "Git repository path")]
        pub repo: &'a Option<String>,
    }

    /// Clone a repository
    pub fn clone(
        params: &Parameters,
        #[arg(help = "Repository URL")]
        url: String,
        #[arg(help = "Target directory")]
        target: Option<String>,
    ) {
        println!("Cloning: {}", url);
        if let Some(t) = target {
            println!("Target: {}", t);
        }
        if let Some(repo) = params.repo {
            println!("Repo path: {}", repo);
        }
        if *params.super_.verbose {
            println!("Verbose: detailed clone output");
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
            println!("Repo: {}", repo);
        }
        if *params.super_.verbose {
            println!("Verbose: showing commit details");
        }
    }

    /// Push changes
    pub fn push(
        params: &Parameters,
        #[arg(long, help = "Remote name")]
        remote: Option<String>,
        #[arg(long, help = "Branch name")]
        branch: Option<String>,
    ) {
        let remote = remote.unwrap_or_else(|| "origin".to_string());
        let branch = branch.unwrap_or_else(|| "main".to_string());
        
        println!("Pushing to {}/{}", remote, branch);
        
        if *params.super_.verbose {
            println!("Verbose: detailed push output");
        }
    }

    /// Advanced git operations
    #[command(about = "Advanced git commands")]
    pub mod advanced {
        /// Rebase branch
        pub fn rebase(
            params: &Parameters,
            #[arg(help = "Base branch")]
            base: String,
        ) {
            println!("Rebasing onto: {}", base);
            
            // Access grandparent parameters
            if let Some(repo) = params.super_.repo {
                println!("Repo: {}", repo);
            }
            if *params.super_.super_.verbose {
                println!("Verbose: detailed rebase output");
            }
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
