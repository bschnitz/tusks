use tusks::tusks;

mod deploy;

#[tusks(root)]
#[command(about = "Async external module test")]
pub mod cli {
    #[command(about = "Deployment commands")]
    pub use crate::deploy::cli as deploy;
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
