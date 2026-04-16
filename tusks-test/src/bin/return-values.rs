use tusks::tusks;

#[tusks(root)]
#[command(about = "Test return values and exit codes", version = "1.0.0")]
pub mod cli {
    /// Command with no return value (implicit success)
    pub fn success() {
        println!("Operation completed successfully");
    }

    /// Command returning u8 exit code
    pub fn check_health(
        #[arg(long, help = "Simulate failure")]
        fail: bool,
    ) -> u8 {
        println!("Running health check...");
        
        if fail {
            println!("✗ Health check failed");
            1
        } else {
            println!("✓ Health check passed");
            0
        }
    }

    /// Command returning Option<u8>
    pub fn validate(
        #[arg(help = "File to validate")]
        file: String,
        #[arg(long, help = "Strict mode")]
        strict: bool,
    ) -> Option<u8> {
        println!("Validating file: {}", file);
        
        if file.ends_with(".invalid") {
            println!("✗ Invalid file format");
            Some(1)  // Error
        } else if file.ends_with(".warning") && strict {
            println!("⚠ File has warnings (strict mode)");
            Some(2)  // Warning code
        } else if file.ends_with(".warning") {
            println!("⚠ File has warnings (non-strict)");
            Some(0)  // Success despite warnings
        } else {
            println!("✓ File is valid");
            Some(0)  // Success
        }
    }

    /// Test module with different return types
    #[command(about = "Test commands")]
    pub mod test {
        /// Test that always succeeds (no return)
        pub fn pass() {
            println!("Test passed");
        }

        /// Test that can fail with u8
        pub fn run(
            #[arg(long, help = "Fail the test")]
            fail: bool,
        ) -> u8 {
            if fail {
                println!("✗ Test failed");
                1
            } else {
                println!("✓ Test passed");
                0
            }
        }

        /// Test with optional return
        pub fn optional(
            #[arg(help = "Test name")]
            name: String,
        ) -> Option<u8> {
            println!("Running test: {}", name);
            
            if name == "fail" {
                println!("✗ Test failed");
                Some(1)
            } else if name == "warn" {
                println!("⚠ Test passed with warnings");
                Some(2)
            } else if name == "skip" {
                println!("○ Test skipped");
                None  // No exit code - defaults to 0
            } else {
                println!("✓ Test passed");
                Some(0)
            }
        }
    }

    /// Process module demonstrating different exit codes
    #[command(about = "Process operations")]
    pub mod process {
        /// Start a process
        pub fn start(
            #[arg(help = "Process name")]
            name: String,
            #[arg(long, help = "Simulate startup failure")]
            fail: bool,
        ) -> u8 {
            println!("Starting process: {}", name);
            
            if fail {
                println!("✗ Failed to start process");
                1
            } else {
                println!("✓ Process started successfully");
                0
            }
        }

        /// Stop a process
        pub fn stop(
            #[arg(help = "Process name")]
            name: String,
            #[arg(long, help = "Force stop")]
            force: bool,
        ) -> Option<u8> {
            println!("Stopping process: {}", name);
            
            if name == "critical" && !force {
                println!("✗ Cannot stop critical process without --force");
                Some(1)
            } else if name == "nonexistent" {
                println!("⚠ Process not found");
                Some(2)
            } else if name == "unknown" {
                println!("○ Process status unknown, assuming stopped");
                None  // No clear exit code - defaults to 0
            } else {
                println!("✓ Process stopped");
                Some(0)
            }
        }

        /// Check process status (always succeeds)
        pub fn status(
            #[arg(help = "Process name")]
            name: String,
        ) {
            println!("Status of {}: running", name);
        }
    }

    /// Deploy module with various exit scenarios
    #[command(about = "Deployment operations")]
    pub mod deploy {
        /// Deploy with validation
        pub fn start(
            #[arg(help = "Version to deploy")]
            version: String,
            #[arg(long, help = "Skip validation")]
            skip_validation: bool,
        ) -> Option<u8> {
            println!("Deploying version: {}", version);
            
            if !skip_validation && version.starts_with("0.") {
                println!("✗ Cannot deploy beta version without --skip-validation");
                Some(1)
            } else if version == "broken" {
                println!("⚠ Deploying known broken version");
                Some(2)
            } else if version == "unknown" {
                println!("○ Version validation skipped");
                None  // No validation result - defaults to 0
            } else {
                println!("✓ Deployment successful");
                Some(0)
            }
        }

        /// Rollback deployment
        pub fn rollback(
            #[arg(long, help = "Steps to rollback")]
            steps: Option<u32>,
        ) -> u8 {
            let steps = steps.unwrap_or(1);
            
            if steps > 10 {
                println!("✗ Cannot rollback more than 10 steps");
                1
            } else {
                println!("✓ Rolled back {} step(s)", steps);
                0
            }
        }
    }

    /// Commands demonstrating custom exit codes
    #[command(about = "Custom exit codes")]
    pub mod custom {
        /// Command with specific exit code
        pub fn exit(
            #[arg(help = "Exit code to return")]
            code: u8,
        ) -> u8 {
            println!("Exiting with code: {}", code);
            code
        }

        /// Command returning multiple possible codes
        pub fn multi(
            #[arg(help = "Scenario to test")]
            scenario: String,
        ) -> Option<u8> {
            match scenario.as_str() {
                "success" => {
                    println!("Success scenario");
                    Some(0)
                }
                "warning" => {
                    println!("Warning scenario");
                    Some(2)
                }
                "error" => {
                    println!("Error scenario");
                    Some(1)
                }
                "critical" => {
                    println!("Critical error");
                    Some(3)
                }
                "none" => {
                    println!("No exit code");
                    None
                }
                _ => {
                    println!("Unknown scenario");
                    Some(255)
                }
            }
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
