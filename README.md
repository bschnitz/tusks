# Tusks Macro Documentation

## 1. Motivation

The tusks macro is designed to simplify the creation of complex command-line interfaces (CLIs) in Rust by providing a declarative approach to defining commands, subcommands, and shared parameters. It eliminates the need for manual clap setup and boilerplate code, allowing developers to focus on business logic rather than CLI framework configuration.

### Why use tusks?

- **Declarative Syntax**: Define your CLI structure using Rust attributes and modules
- **Automatic Parameter Chaining**: Share parameters across commands and subcommands
- **Hierarchical Structure**: Naturally map Rust module structure to CLI command hierarchy
- **Type Safety**: Leverage Rust's type system for argument validation
- **Reduced Boilerplate**: No need to manually configure clap parsers
- **External Module Integration**: Easily include subcommands from other files

The macro processes your Rust code structure and automatically generates the corresponding clap-based CLI, handling argument parsing, subcommand routing, and parameter sharing.

## 2. Simple CLI Example

Let's start with a basic CLI that has two commands without subcommands or shared parameters.

### Basic Structure

```rust
use tusks::tusks;

#[tusks(root)]
#[command(
    about = "Simple CLI example",
    version = "1.0.0",
    author = "Your Name"
)]
pub mod cli {
    // Command 1: Hello command
    #[command(about = "Prints a greeting")]
    pub fn hello() {
        println!("Hello, world!");
    }

    // Command 2: Add command with arguments
    #[command(about = "Adds two numbers")]
    pub fn add(#[arg(long)] a: i32, #[arg(long)] b: i32) -> u8 {
        println!("{} + {} = {}", a, b, a + b);
        0  // Return exit code
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

### Required Elements

1. **Root Module**: Must be marked with `#[tusks(root)]`
2. **Public Module**: The module must be `pub`
3. **Command Functions**: Public functions marked with `#[command(...)]`
4. **Argument Attributes**: Use `#[arg(...)]` for command-line arguments
5. **Execution**: Call `exec_cli()` to run the CLI

### Generated CLI

This code generates a CLI with the following structure:

```bash
$ cargo run -- --help
Simple CLI example

USAGE:
    cli [OPTIONS] <COMMAND>

OPTIONS:
    --help       Print help information
    --version    Print version information

COMMANDS:
    add     Adds two numbers
    hello   Prints a greeting
```

### Command Usage Examples

```bash
# Run hello command
$ cargo run -- hello
Hello, world!

# Run add command with arguments
$ cargo run -- add --a 5 --b 3
5 + 3 = 8

# Get help for specific command
$ cargo run -- add --help
Adds two numbers

USAGE:
    cli add [OPTIONS] --a <a> --b <b>

OPTIONS:
    --a <a>    First number
    --b <b>    Second number
    --help     Print help information
```

## 3. Function Return Values

### Requirements and Restrictions

Functions that become CLI commands have specific return type requirements:

1. **Unit Type (`()`)**: No return value, function executes and exits with code 0
2. **`u8`**: Return an exit code (0 for success, non-zero for errors)
3. **`Option<u8>`**: Return an optional exit code (None for success, Some(code) for errors)

### Why These Restrictions?

- **Consistent Exit Codes**: Provides a standard way to communicate command success/failure
- **Simple Error Handling**: Makes it easy to propagate error conditions
- **Clap Integration**: Matches clap's expected return value patterns

### Examples

```rust
// Returns unit type (implicit 0 exit code)
#[command(about = "Unit return example")]
pub fn unit_example() {
    println!("This command succeeds");
}

// Returns explicit exit code
#[command(about = "Explicit exit code")]
pub fn explicit_code() -> u8 {
    println!("This command returns 1");
    1  // Error code
}

// Returns optional exit code
#[command(about = "Optional exit code")]
pub fn optional_code() -> Option<u8> {
    println!("This command may fail");
    Some(2)  // Error code
}
```

## 4. Subcommands through Nested Modules

Subcommands are created by nesting modules within your CLI module. Each nested module becomes a subcommand group.

### Structure

```rust
use tusks::tusks;

#[tusks(root)]
#[command(about = "Subcommand example")]
pub mod cli {
    // Root command
    #[command(about = "Root command")]
    pub fn root() {
        println!("Root command executed");
    }

    // Subcommand group: user
    #[command(about = "User management commands")]
    pub mod user {
        // Subcommand: create
        #[command(about = "Create a new user")]
        pub fn create(#[arg(long)] name: String) {
            println!("Creating user: {}", name);
        }

        // Subcommand: delete
        #[command(about = "Delete a user")]
        pub fn delete(#[arg(long)] id: u32) {
            println!("Deleting user with ID: {}", id);
        }
    }

    // Another subcommand group: system
    #[command(about = "System management commands")]
    pub mod system {
        #[command(about = "Restart the system")]
        pub fn restart() {
            println!("System restarting...");
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

### Generated CLI Structure

```bash
$ cargo run -- --help
Subcommand example

USAGE:
    cli [OPTIONS] <COMMAND>

OPTIONS:
    --help    Print help information

COMMANDS:
    root      Root command
    system    System management commands
    user      User management commands
```

### Subcommand Usage

```bash
# Root command
$ cargo run -- root
Root command executed

# User subcommand
$ cargo run -- user create --name alice
Creating user: alice

# System subcommand
$ cargo run -- system restart
System restarting...

# Help for subcommand
$ cargo run -- user --help
User management commands

USAGE:
    cli user <COMMAND>

COMMANDS:
    create    Create a new user
    delete    Delete a user
```

## 5. Parameters Struct

The `Parameters` struct allows you to define shared configuration that can be accessed by multiple commands within the same module.

### Basic Usage

```rust
use tusks::tusks;

#[tusks(root)]
#[command(about = "Parameters example")]
pub mod cli {
    // Parameters struct for shared configuration
    pub struct Parameters<'a> {
        #[arg(long)]
        pub config_file: &'a String,
        
        #[arg(long, default_value = "false")]
        pub verbose: &'a bool,
    }

    // Command using parameters
    #[command(about = "Show configuration")]
    pub fn show_config(params: &Parameters) {
        println!("Config file: {}", params.config_file);
        if *params.verbose {
            println!("[VERBOSE] Verbose mode enabled");
        }
    }

    // Another command using the same parameters
    #[command(about = "Process data")]
    pub fn process_data(params: &Parameters, #[arg(long)] input: String) {
        println!("Processing {} with config: {}", input, params.config_file);
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

### Parameter Chaining

Parameters can be defined at different module levels, creating a hierarchical chain:

```rust
use tusks::tusks;

#[tusks(root)]
#[command(about = "Parameter chaining example")]
pub mod cli {
    // Root parameters (accessible by all commands)
    pub struct Parameters<'a> {
        #[arg(long)]
        pub root_config: &'a String,
    }

    #[command(about = "Root command")]
    pub fn root_command(params: &Parameters) {
        println!("Root config: {}", params.root_config);
    }

    // Submodule with its own parameters
    #[command(about = "Level 1 submodule")]
    pub mod level1 {
        // Level 1 parameters (can access root via super_)
        pub struct Parameters<'a> {
            #[arg(long)]
            pub level1_setting: &'a i32,
        }

        #[command(about = "Level 1 command")]
        pub fn level1_command(params: &Parameters) {
            // Access root parameters through super_
            println!("Level 1 setting: {}", params.level1_setting);
            println!("Root config: {}", params.super_.root_config);
        }

        // Deeper nested module
        #[command(about = "Level 2 submodule")]
        pub mod level2 {
            // Level 2 parameters
            pub struct Parameters<'a> {
                #[arg(long)]
                pub level2_flag: &'a bool,
            }

            #[command(about = "Level 2 command")]
            pub fn level2_command(params: &Parameters) {
                // Access level1 parameters via super_
                // Access root parameters via super_.super_
                println!("Level 2 flag: {}", params.level2_flag);
                println!("Level 1 setting: {}", params.super_.level1_setting);
                println!("Root config: {}", params.super_.super_.root_config);
            }
        }
    }
}
```

### Key Rules

1. **Name**: Must be exactly `Parameters` (This name is thus reseverd in all
   tusks-modules and submodules).
2. **Visibility**: Must be `pub`
3. **Field Types**: All fields must be reference types (`&T`)
4. **No super_ Field**: Cannot contain a `super_` field (it's added automatically)
5. **Optional**: Only needed if you want to share parameters

Even if You do not create the struct manually, it will always be added
automatically. This means, that You can always use `_super` to navigate to the
toplevel `Parameters`-structs, even if You didn't specify intermediate levels.

## 6. Subcommands through External Modules

External modules allow you to define subcommands in separate files and include them in your CLI.

### File Structure

```
src/
├── main.rs          # Main CLI
└── external.rs      # External subcommand module
```

### Syntax and Implementation

**main.rs:**
```rust
use tusks::tusks;

#[tusks(root)]
#[command(about = "External module example")]
pub mod cli {
    // Include external module as subcommand
    #[command(about = "External user management")]
    pub use crate::external::user as user;
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

**external.rs:**
```rust
use tusks::tusks;

#[tusks()]
#[command(about = "User management commands")]
pub mod user {
    // Reference to parent module (required for parameter chaining)
    pub use crate::cli as parent_;
    
    pub struct Parameters<'a> {
        #[arg(long)]
        pub user_config: &'a String,
    }

    #[command(about = "Create user")]
    pub fn create(params: &Parameters, #[arg(long)] name: String) {
        println!("Creating user {} with config: {}", name, params.user_config);
    }

    #[command(about = "List users")]
    pub fn list(params: &Parameters) {
        println!("Listing users with config: {}", params.user_config);
    }
}
```

### Key Requirements

1. **External Module**: Must be marked with `#[tusks()]` (empty attributes)
2. **Parent Reference**: Must include `pub use crate::parent_module as parent_;`
3. **Public Module**: The module must be `pub`
4. **Parameter Chaining**: Can access parent parameters via `parent_`
5. The subcommand created for an external is equal to it's name or it's alias
   used when including it via `pub use path::to::external as alias`

### Usage

```bash
$ cargo run -- user create --name bob --user-config "users.json"
Creating user bob with config: users.json

$ cargo run -- user list --user-config "users.json" 
Listing users with config: users.json
```

## 7. Relationship with Clap

The tusks macro is built on top of the clap library and automatically generates clap-based CLI structures. The syntax for the `#[command(...)]` and `#[arg(..)]` attributes is therefore the same as for clap.

The `#[command(...)]`-Attribute can be specified on the root-module, it's
submodules (including external modules) and functions.

### Integration Example

```rust
use tusks::tusks;

#[tusks(root)]
#[command(
    about = "Advanced clap integration",
    version = "1.0.0",
    arg_required_else_help = true
)]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long, help = "Configuration file path")]
        pub config: &'a String,
        
        #[arg(short, long, default_value = "false")]
        pub verbose: &'a bool,
    }

    #[command(
        about = "Process data with custom options",
        long_about = "This command processes input data with various options"
    )]
    pub fn process(
        params: &Parameters,
        #[arg(long, value_name = "INPUT", help = "Input file path")]
        input: String,
        
        #[arg(long, value_parser = parse_number, help = "Processing count")]
        count: u32
    ) -> u8 {
        println!("Processing {} with count {}", input, count);
        if *params.verbose {
            println!("[VERBOSE] Using config: {}", params.config);
        }
        0
    }
}

// Custom parser function
fn parse_number(s: &str) -> Result<u32, String> {
    s.parse().map_err(|_| "Must be a positive number".into())
}
```

## 7. Exclude Functions / Modules from the CLI

Private functions and modules are excluded from the CLI by default. All other
functions and modules can be marked with the `#[skip]`-Attribute to exclude
them. Making them invisible to the tusks-macro (and also removing any
restrictions which would apply otherwise).
