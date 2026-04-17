use tusks::tusks;

#[tusks(root)]
#[command(about = "Result return type test", version = "1.0.0")]
pub mod cli {
    /// Succeeds with Ok(())
    pub fn succeed() -> Result<(), String> {
        println!("Success!");
        Ok(())
    }

    /// Fails with an error message
    pub fn fail() -> Result<(), String> {
        Err("something went wrong".to_string())
    }

    /// Returns Ok with exit code
    pub fn check() -> Result<u8, String> {
        println!("Check passed");
        Ok(0)
    }

    /// Returns Ok with custom exit code
    pub fn check_fail() -> Result<u8, String> {
        Ok(42)
    }

    /// Returns Result<Option<u8>, E>
    pub fn maybe(#[arg(long)] fail: bool) -> Result<Option<u8>, String> {
        if fail {
            Err("not ok".to_string())
        } else {
            println!("All good");
            Ok(Some(0))
        }
    }

    /// Doc comment used as help text
    pub fn documented(message: String) {
        println!("Message: {}", message);
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
