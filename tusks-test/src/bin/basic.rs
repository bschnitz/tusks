use tusks::tusks;

#[tusks(root)]
#[command(
    about = "Basic CLI test",
    version = "1.0.0"
)]
pub mod cli {
    /// Say hello
    pub fn hello(
        #[arg(help = "Name to greet")]
        name: String,
    ) {
        println!("Hello, {}!", name);
    }
    
    /// Say goodbye
    pub fn goodbye(
        #[arg(help = "Name to say goodbye to")]
        name: String,
        #[arg(long, help = "Be formal")]
        formal: bool,
    ) {
        if formal {
            println!("Farewell, {}!", name);
        } else {
            println!("Bye, {}!", name);
        }
    }
    
    /// Echo a message
    pub fn echo(
        #[arg(help = "Message to echo")]
        message: String,
        #[arg(short, long, help = "Number of times to repeat")]
        count: Option<u32>,
    ) {
        let count = count.unwrap_or(1);
        for _ in 0..count {
            println!("{}", message);
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
