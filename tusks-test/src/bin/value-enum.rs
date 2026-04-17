use tusks::tusks;

#[tusks(root)]
#[command(about = "ValueEnum test", version = "1.0.0")]
pub mod cli {
    #[derive(Clone, ::tusks::clap::ValueEnum)]
    pub enum Color {
        Auto,
        Always,
        Never,
    }

    #[derive(Clone, ::tusks::clap::ValueEnum)]
    pub enum Format {
        Json,
        Text,
        Yaml,
    }

    /// Paint with color
    pub fn paint(
        #[arg(long, default_value = "auto")]
        color: Color,
        message: String,
    ) {
        let color_str = match color {
            Color::Auto => "auto",
            Color::Always => "always",
            Color::Never => "never",
        };
        println!("color={} message={}", color_str, message);
    }

    /// Output in format
    pub fn output(
        #[arg(long)]
        format: Format,
        data: String,
    ) {
        let fmt = match format {
            Format::Json => "json",
            Format::Text => "text",
            Format::Yaml => "yaml",
        };
        println!("format={} data={}", fmt, data);
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0))
}
