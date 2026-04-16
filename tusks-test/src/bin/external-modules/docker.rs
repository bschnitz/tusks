use tusks::tusks;

#[tusks()]
pub mod cli {
    // Reference to parent module
    pub use crate::cli as parent_;

    pub struct Parameters<'a> {
        #[arg(long, help = "Docker host")]
        pub host: &'a Option<String>,
    }

    /// Build Docker image
    pub fn build(
        params: &Parameters,
        #[arg(help = "Build context path")]
        context: String,
        #[arg(short, long, help = "Image tag")]
        tag: Option<String>,
    ) {
        println!("Building image from: {}", context);
        if let Some(t) = tag {
            println!("Tag: {}", t);
        }
        if let Some(host) = params.host {
            println!("Docker host: {}", host);
        }
        if *params.super_.verbose {
            println!("Verbose: detailed build output");
        }
    }

    /// Run container
    pub fn run(
        params: &Parameters,
        #[arg(help = "Image name")]
        image: String,
        #[arg(short, long, help = "Container name")]
        name: Option<String>,
    ) {
        println!("Running image: {}", image);
        if let Some(n) = name {
            println!("Container name: {}", n);
        }
        if *params.super_.verbose {
            println!("Verbose: detailed run output");
        }
    }

    /// Container operations
    #[command(about = "Container management")]
    pub mod container {
        /// List containers
        pub fn list(
            params: &Parameters,
            #[arg(short, long, help = "Show all containers")]
            all: bool,
        ) {
            if all {
                println!("Listing all containers");
            } else {
                println!("Listing running containers");
            }
            
            // Access grandparent verbose
            if *params.super_.super_.verbose {
                println!("Verbose: detailed container info");
            }
        }

        /// Stop container
        pub fn stop(
            #[arg(help = "Container ID or name")]
            container: String,
        ) {
            println!("Stopping container: {}", container);
        }
    }

    /// Image operations
    #[command(about = "Image management")]
    pub mod image {
        /// List images
        pub fn list(params: &Parameters) {
            println!("Listing images");
            if *params.super_.super_.verbose {
                println!("Verbose: detailed image info");
            }
        }

        /// Remove image
        pub fn remove(
            #[arg(help = "Image ID or name")]
            image: String,
            #[arg(short, long, help = "Force removal")]
            force: bool,
        ) {
            if force {
                println!("Force removing image: {}", image);
            } else {
                println!("Removing image: {}", image);
            }
        }
    }
}
