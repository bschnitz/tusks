use tusks::tusks;

#[tusks(root)]
#[command(about = "Global arg test", version = "1.0.0")]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long, global = true)]
        pub verbose: &'a bool,
    }

    pub fn root_cmd(params: &Parameters) {
        println!("root verbose={}", params.verbose);
    }

    #[command(about = "Sub operations")]
    pub mod sub {
        pub fn action(params: &Parameters) {
            println!("sub verbose={}", params.super_.verbose);
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
