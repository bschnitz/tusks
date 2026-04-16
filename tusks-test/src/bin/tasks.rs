use tusks::tusks;

#[tusks(root, tasks(max_groupsize=5, max_depth=20, separator=".", use_colors=false))]
#[command(
    about = "Task management binary",
    long_about = "A binary for managing tasks with git and docker submodules",
    version = "1.0.0",
    author = "Task Manager",
)]
pub mod tasks {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub root_param: &'a Option<String>,
        
        #[arg(short, long)]
        pub verbose: &'a bool,
    }

    /// Git submodule
    #[command(about = "Git operations")]
    pub mod git {
        /// Clone repository
        #[command(about = "Clone a git repository")]
        pub fn clone(url: String, path: Option<String>) {
            println!("Cloning {} to {:?}", url, path.unwrap_or("current directory".to_string()));
        }

        /// Commit changes
        #[command(about = "Commit changes")]
        pub fn commit(message: String) {
            println!("Committing with message: {}", message);
        }
    }

    /// Docker submodule
    #[command(about = "Docker operations")]
    pub mod docker {
        /// Build image
        #[command(about = "Build Docker image")]
        pub fn build(context: String, tag: Option<String>) {
            println!("Building Docker image from {} with tag {:?}", context, tag);
        }

        /// Run container
        #[command(about = "Run Docker container")]
        pub fn run(image: String, args: Vec<String>) {
            println!("Running container {} with args: {:?}", image, args);
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(tasks::exec_cli().unwrap_or(0) as u8)
}
