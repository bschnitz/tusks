use tusks::tusks;

#[tusks(root)]
#[command(about = "Test all #[arg] attribute features", version = "1.0.0")]
pub mod cli {

    /// Test basic argument types
    pub fn types(
        #[arg(help = "A string argument")]
        text: String,
        
        #[arg(help = "An unsigned integer")]
        number: u32,
        
        #[arg(help = "A signed integer")]
        signed: i32,
        
        #[arg(help = "A float")]
        float: f64,
        
        #[arg(long, help = "A boolean flag")]
        flag: bool,
    ) {
        println!("String: {}", text);
        println!("Number (u32): {}", number);
        println!("Signed (i32): {}", signed);
        println!("Float (f64): {}", float);
        println!("Flag (bool): {}", flag);
    }

    /// Test short and long flags
    pub fn flags(
        #[arg(short, long, help = "Verbose output")]
        verbose: bool,
        
        #[arg(short = 'q', long = "quiet", help = "Quiet mode")]
        quiet: bool,
        
        #[arg(short, long = "debug", help = "Debug mode")]
        debug: bool,
    ) {
        println!("Verbose: {}", verbose);
        println!("Quiet: {}", quiet);
        println!("Debug: {}", debug);
    }

    /// Test optional arguments
    pub fn optional(
        #[arg(help = "Required argument")]
        required: String,
        
        #[arg(long, help = "Optional string")]
        opt: Option<String>,
        
        #[arg(long, help = "Optional number")]
        number: Option<u32>,
        
        #[arg(long, help = "Optional path")]
        path: Option<::std::path::PathBuf>,
    ) {
        println!("Required: {}", required);
        println!("Optional: {:?}", opt);
        println!("Number: {:?}", number);
        println!("Path: {:?}", path);
    }

    /// Test default values
    pub fn defaults(
        #[arg(long, default_value = "default.txt", help = "File name")]
        file: String,
        
        #[arg(long, default_value_t = 10, help = "Count")]
        count: u32,
        
        #[arg(long, default_value = "info", help = "Log level")]
        level: String,
    ) {
        println!("File: {}", file);
        println!("Count: {}", count);
        println!("Level: {}", level);
    }

    /// Test multiple values
    pub fn multiple(
        #[arg(long, help = "Multiple files", num_args = 1..)]
        files: Vec<String>,
        
        #[arg(long, help = "Tags (comma-separated)", value_delimiter = ',')]
        tags: Vec<String>,
        
        #[arg(long, help = "Optional items")]
        items: Vec<String>,
    ) {
        println!("Files: {:?}", files);
        println!("Tags: {:?}", tags);
        println!("Items: {:?}", items);
    }

    /// Test value names and help
    pub fn naming(
        #[arg(value_name = "INPUT_FILE", help = "Input file path")]
        input: std::path::PathBuf,
        
        #[arg(long, value_name = "OUTPUT_FILE", help = "Output file path")]
        output: std::path::PathBuf,
        
        #[arg(
            long, 
            value_name = "PORT",
            help = "Server port",
            long_help = "The port number on which the server will listen.\nMust be between 1024 and 65535."
        )]
        port: u16,
    ) {
        println!("Input: {:?}", input);
        println!("Output: {:?}", output);
        println!("Port: {}", port);
    }

    /// Test custom value parser
    pub fn parser(
        #[arg(long, value_parser = crate::cli::parse_percentage, help = "Percentage (0-100)")]
        percentage: u8,
        
        #[arg(long, value_parser = crate::cli::parse_port, help = "Port (1024-65535)")]
        port: u16,
    ) {
        println!("Percentage: {}%", percentage);
        println!("Port: {}", port);
    }

    /// Test conflicts and requirements
    pub fn conflicts(
        #[arg(long, help = "Use verbose mode", conflicts_with = "quiet")]
        verbose: bool,
        
        #[arg(long, help = "Use quiet mode", conflicts_with = "verbose")]
        quiet: bool,
        
        #[arg(long, help = "Enable feature", requires = "config")]
        feature: bool,
        
        #[arg(long, help = "Config file (required if --feature is used)")]
        config: Option<String>,
    ) {
        println!("Verbose: {}", verbose);
        println!("Quiet: {}", quiet);
        println!("Feature: {}", feature);
        println!("Config: {:?}", config);
    }

    /// Test environment variables
    pub fn environment(
        #[arg(long, env = "USER", help = "Username (from USER env var)")]
        user: Option<String>,
        
        #[arg(long, env = "HOME", help = "Home directory (from HOME env var)")]
        home: Option<std::path::PathBuf>,
        
        #[arg(long, env = "LOG_LEVEL", default_value = "info", help = "Log level")]
        log_level: String,
    ) {
        println!("User: {:?}", user);
        println!("Home: {:?}", home);
        println!("Log level: {}", log_level);
    }

    /// Test hidden arguments
    pub fn hidden(
        #[arg(help = "Visible argument")]
        visible: String,
        
        #[arg(long, hide = true, help = "Hidden debug flag")]
        debug_secret: bool,
        
        #[arg(long, help = "Normal flag")]
        normal: bool,
    ) {
        println!("Visible: {}", visible);
        println!("Debug secret: {}", debug_secret);
        println!("Normal: {}", normal);
    }

    /// Test action types (counting)
    pub fn actions(
        #[arg(short, long, action = ::tusks::clap::ArgAction::Count, help = "Verbosity level (-v, -vv, -vvv)")]
        verbose: u8,
        
        #[arg(help = "Message to display")]
        message: String,
    ) {
        let level = match verbose {
            0 => "normal",
            1 => "verbose",
            2 => "very verbose",
            _ => "debug",
        };
        println!("Verbosity: {} (level {})", level, verbose);
        println!("Message: {}", message);
    }

    /// Test positional vs named arguments
    pub fn positional(
        #[arg(help = "First positional argument")]
        first: String,
        
        #[arg(help = "Second positional argument")]
        second: String,
        
        #[arg(long, help = "Named argument")]
        named: Option<String>,
        
        #[arg(help = "Optional third positional")]
        third: Option<String>,
    ) {
        println!("First: {}", first);
        println!("Second: {}", second);
        println!("Named: {:?}", named);
        println!("Third: {:?}", third);
    }

    /// Test value hints for shell completion
    pub fn hints(
        #[arg(long, value_hint = ::tusks::clap::ValueHint::FilePath, help = "Input file")]
        input: std::path::PathBuf,
        
        #[arg(long, value_hint = ::tusks::clap::ValueHint::DirPath, help = "Output directory")]
        output_dir: std::path::PathBuf,
        
        #[arg(long, value_hint = ::tusks::clap::ValueHint::ExecutablePath, help = "Command to run")]
        command: Option<std::path::PathBuf>,
        
        #[arg(long, value_hint = ::tusks::clap::ValueHint::Url, help = "Remote URL")]
        url: Option<String>,
    ) {
        println!("Input: {:?}", input);
        println!("Output dir: {:?}", output_dir);
        println!("Command: {:?}", command);
        println!("URL: {:?}", url);
    }

    /// Test number ranges
    pub fn ranges(
        #[arg(long, value_parser = ::tusks::clap::value_parser!(u32).range(1..=100), help = "Number between 1-100")]
        small: u32,
        
        #[arg(long, value_parser = ::tusks::clap::value_parser!(i32).range(-50..=50), help = "Number between -50 and 50")]
        signed: i32,
    ) {
        println!("Small: {}", small);
        println!("Signed: {}", signed);
    }

    // Helper functions for custom parsers
    #[skip]
    pub fn parse_percentage(s: &str) -> Result<u8, String> {
        let val: u8 = s.parse()
            .map_err(|_| format!("'{}' is not a valid number", s))?;
        
        if val > 100 {
            return Err("Percentage must be between 0 and 100".to_string());
        }
        
        Ok(val)
    }

    #[skip]
    pub fn parse_port(s: &str) -> Result<u16, String> {
        let port: u16 = s.parse()
            .map_err(|_| format!("'{}' is not a valid port number", s))?;
        
        if port < 1024 {
            return Err("Port must be 1024 or higher".to_string());
        }
        
        Ok(port)
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
