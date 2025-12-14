# Tusks

Tusks allows you to define CLIs easily and idiomatically through a Rust module and function structure.

If You just want a quick example head over to the [Comprehensive Example](#comprehensive-example) section.

## Table of Contents

- [Motivation](#motivation)
- [Installation](#installation)
- [Core Concepts](#core-concepts)
- [Relationship with Clap](#relationship-with-clap)
- [Features and Examples](#features-and-examples)
  - [1. Simple Root Module Definition](#1-simple-root-module-definition)
  - [2. Nested Modules (Subcommands)](#2-nested-modules-subcommands)
  - [3. Root Parameters with Parameters Struct](#3-root-parameters-with-parameters-struct)
  - [4. Module-Level Parameters](#4-module-level-parameters)
  - [5. External Modules](#5-external-modules)
  - [6. Return Values and Exit Codes](#6-return-values-and-exit-codes)
  - [7. Various Argument Types](#7-various-argument-types)
  - [8. Custom Value Parsers](#8-custom-value-parsers)
  - [9. Tasks Mode (Ruby Rake-Style)](#9-tasks-mode-ruby-rake-style)
  - [10. Command Attributes for Documentation](#10-command-attributes-for-documentation)
  - [11. Default Functions for Modules](#11-default-functions-for-modules)
- [Comprehensive Example](#comprehensive-example)
- [License](#license)
- [Contributions](#contributions)
- [Trivia](#trivia)

## Motivation

Creating complex CLI applications with nested commands and shared parameters can quickly become unwieldy. Tusks solves this problem by providing a declarative syntax for CLI structures that:

- **Naturally maps hierarchical command structures**
- **Automatically manages parameter chaining** across multiple levels
- **Uses Clap** under the hood but eliminates boilerplate code
- **Supports modular organization** through external modules
- **Guarantees type safety** through Rust/Clap's type system

Instead of manually managing Clap subcommands and creating match statements, you simply define modules and functions – Tusks takes care of the rest.

> **Want to see it in action?** Check out the [comprehensive example](#comprehensive-example) at the end of this document.

## Installation

```toml
[dependencies]
tusks = "2.0"
clap = { version = "4.0", features = ["derive"] }
```

## Core Concepts

Tusks is based on four main concepts:

1. **Modules as Commands/Subcommands**: Rust modules automatically become CLI commands. Modules serve to hierarchically group functions as subcommands.
2. **Functions as Commands/Subcommands**: Public functions in modules become executable CLI commands/subcommands. Function arguments are automatically translated into CLI parameters.
3. **Parameters Struct for Command Arguments**: The `Parameters` struct defines arguments at the module level that apply to the respective command/subcommand. These arguments are automatically available to all underlying subcommands.
4. **External Modules**: External modules, i.e., modules in other files, can be easily integrated into the current CLI structure, even recursively!

The CLI is started with `cli::exec_cli()`, which parses the command line arguments and executes the corresponding commands. The function always returns `Option<u8>`, which can be used as an exit code.

## Relationship with Clap

Tusks is a high-level wrapper around [Clap](https://docs.rs/clap/), the popular CLI parsing framework for Rust. Many of Clap's features are retained.

- `#[arg()]` attributes for argument configuration
- `#[command()]` attributes for subcommand descriptions
- Data types are parsed as in Clap
- Automatic help generation
- Type-safe parsing

Tusks generates Clap code internally.

## Features and Examples

### 1. Simple Root Module Definition

The `#[tusks(root)]` attribute marks a module as the CLI entry point. The CLI is started with `cli::exec_cli()`.

```rust
use tusks::tusks;

#[tusks(root)]
#[command(
    about = "My CLI tool",
    version = "1.0.0"
)]
pub mod cli {
    /// A simple command
    pub fn hello(name: String) {
        println!("Hello, {}!", name);
    }
}

fn main() -> std::process::ExitCode {
    // exec_cli() starts the CLI and returns Option<u8>
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

**Usage:**
```bash
$ my-cli hello Alice
Hello, Alice!
```

#### Visibility and #[skip]

Only **public** (`pub`) modules and functions are used for CLI construction. Private functions are automatically ignored:

```rust
#[tusks(root)]
pub mod cli {
    /// This command is available
    pub fn public_command() {
        println!("This is a CLI command");
    }
    
    /// This function is NOT available (not pub)
    fn private_helper() {
        println!("This is not a CLI command");
    }
}
```

With the `#[skip]` attribute, you can also exclude public functions from CLI parsing:

```rust
#[tusks(root)]
pub mod cli {
    /// CLI command
    pub fn deploy(target: String) {
        let config = load_config();
        // ... deployment logic ...
    }
    
    /// Helper function - not available as CLI command
    #[skip]
    pub fn load_config() -> Config {
        // This function is public (for other modules),
        // but not a CLI command
        Config::from_file("config.toml")
    }
}
```

The `#[skip]` attribute also works for modules:

```rust
#[tusks(root)]
pub mod cli {
    /// Available subcommand
    pub mod deploy { /* ... */ }
    
    /// Not available as subcommand
    #[skip]
    pub mod internal_utils { /* ... */ }
}
```

### 2. Nested Modules (Subcommands)

Modules automatically become subcommands and serve to hierarchically group functions.

```rust
#[tusks(root)]
pub mod cli {
    #[command(about = "Database operations")]
    pub mod database {
        /// Migrate database
        pub fn migrate(version: String) {
            println!("Migrating database to version {}", version);
        }
        
        /// Backup database
        pub fn backup(path: String) {
            println!("Backing up database to {}", path);
        }
    }
    
    #[command(about = "Deployment operations")]
    pub mod deploy {
        /// Deploy to staging
        pub fn staging() {
            println!("Deploying to staging environment");
        }
        
        /// Deploy to production
        pub fn production() {
            println!("Deploying to production environment");
        }
    }
}
```

**Usage:**
```bash
$ my-cli database migrate v2.0
Migrating database to version v2.0

$ my-cli deploy production
Deploying to production environment

$ my-cli --help
My CLI tool

Usage: my-cli <COMMAND>

Commands:
  database  Database operations
  deploy    Deployment operations
  help      Print this message or the help of the given subcommand(s)
```

Modules can be nested arbitrarily deep:

```rust
#[tusks(root)]
pub mod cli {
    pub mod cloud {
        pub mod aws {
            pub mod s3 {
                /// Upload file to S3
                pub fn upload(file: String, bucket: String) {
                    println!("Uploading {} to bucket {}", file, bucket);
                }
            }
        }
    }
}
```

**Usage:**
```bash
$ my-cli cloud aws s3 upload file.txt my-bucket
Uploading file.txt to bucket my-bucket
```

With a `Parameters` struct, you can define common parameters that are available to all commands.

```rust
#[tusks(root)]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub verbose: &'a bool,
        
        #[arg(long)]
        pub config: &'a Option<String>,
    }

    /// Command with access to root parameters
    pub fn deploy(params: &Parameters, target: String) {
        if *params.verbose {
            println!("Deploying to {} with config {:?}", 
                     target, params.config);
        }
    }
}
```

**Usage:**
```bash
$ my-cli --verbose --config prod.toml deploy production
Deploying to production with config Some("prod.toml")
```

#### Important Notes on Parameters

- **Optional**: The `Parameters` struct is completely optional. You only need it if you want to define parameters at the current module level or access parent parameters.

- **Lifetime required**: If you define a `Parameters` struct, it must always have the lifetime `<'a>`:
  ```rust
  pub struct Parameters<'a> {  // <'a> is mandatory
      #[arg(long)]
      pub my_param: &'a String,
  }
  ```

- **Automatic `super_` field**: Tusks automatically adds a `super_` field that references the parent Parameters struct. You must **not** define this field yourself:
  ```rust
  pub struct Parameters<'a> {
      #[arg(long)]
      pub my_param: &'a String,
      // NOT: pub super_: &'a ParentParameters<'a>  ❌
  }
  ```

- **Implicit Parameters structs**: Even if you don't define a `Parameters` struct at a level, it exists in the background. This means that `super_.super_` always works to access parameters two levels up:
  ```rust
  pub mod level1 {
      // No Parameters struct defined here
      
      pub mod level2 {
          pub struct Parameters<'a> {
              #[arg(long)]
              pub level2_param: &'a String,
          }
          
          pub fn command(params: &Parameters) {
              // super_.super_ still works!
              println!("{}", params.super_.super_.root_param);
          }
      }
  }
  ```

- **Parameters as function argument**: The `Parameters` struct may only be specified as the **first argument** of a function and is optional. If you don't need it, you can omit it:
  ```rust
  // With Parameters (must be first argument)
  pub fn command1(params: &Parameters, name: String) { }
  
  // Without Parameters
  pub fn command2(name: String, age: u32) { }
  
  // WRONG: Parameters not in first position ❌
  pub fn command3(name: String, params: &Parameters) { }
  ```

### 3. Root Parameters with Parameters Struct

With a `Parameters` struct, you can define common parameters that are available to all commands.

```rust
#[tusks(root)]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub verbose: &'a bool,
        
        #[arg(long)]
        pub config: &'a Option<String>,
    }

    /// Command with access to root parameters
    pub fn deploy(params: &Parameters, target: String) {
        if *params.verbose {
            println!("Deploying to {} with config {:?}", 
                     target, params.config);
        }
    }
}
```

**Usage:**
```bash
$ my-cli --verbose --config prod.toml deploy production
Deploying to production with config Some("prod.toml")
```

Modules automatically become subcommands with their own parameters.

```rust
#[tusks(root)]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub verbose: &'a bool,
    }

    #[command(about = "Database operations")]
    pub mod database {
        pub struct Parameters<'a> {
            #[arg(long)]
            pub connection: &'a String,
        }

        /// Migrate database
        pub fn migrate(params: &Parameters) {
            println!("Migrating database: {}", params.connection);
            
            // Access parent parameters via super_
            if *params.super_.verbose {
                println!("Verbose mode enabled");
            }
        }
    }
}
```

**Usage:**
```bash
$ my-cli --verbose database --connection "localhost:5432" migrate
Migrating database: localhost:5432
Verbose mode enabled
```

### 4. Module-Level Parameters

Each module can define its own `Parameters` struct to define specific arguments for the respective subcommand. These parameters are automatically available to all underlying subcommands.

```rust
#[tusks(root)]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub verbose: &'a bool,
    }

    #[command(about = "Database operations")]
    pub mod database {
        pub struct Parameters<'a> {
            #[arg(long)]
            pub connection: &'a String,
        }

        /// Migrate database
        pub fn migrate(params: &Parameters) {
            println!("Migrating database: {}", params.connection);
            
            // Access parent parameters via super_
            if *params.super_.verbose {
                println!("Verbose mode enabled");
            }
        }
    }
}
```

**Usage:**
```bash
$ my-cli --verbose database --connection "localhost:5432" migrate
Migrating database: localhost:5432
Verbose mode enabled
```

Parameters can be passed through an arbitrary number of levels.

```rust
#[tusks(root)]
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub env: &'a String,
    }

    pub mod services {
        pub struct Parameters<'a> {
            #[arg(long)]
            pub region: &'a String,
        }

        pub mod kubernetes {
            pub struct Parameters<'a> {
                #[arg(long)]
                pub namespace: &'a String,
            }

            /// Deploy to k8s
            pub fn deploy(params: &Parameters, image: String) {
                // Access all levels:
                println!("Environment: {}", params.super_.super_.env);
                println!("Region: {}", params.super_.region);
                println!("Namespace: {}", params.namespace);
                println!("Image: {}", image);
            }
        }
    }
}
```

**Usage:**
```bash
$ my-cli --env production services --region eu-west-1 kubernetes --namespace default deploy my-app:v1.0
Environment: production
Region: eu-west-1
Namespace: default
Image: my-app:v1.0
```

### 5. External Modules

External modules allow distributing CLI structures across multiple files. This is particularly useful for organizing large CLIs and promoting code reusability.

An external module differs from the root module in that it does **not** have the `root` flag in the `#[tusks()]` attribute. Instead, it must include a `parent_` reference to its parent module to enable parameter chaining.

**src/main.rs:**
```rust
#[tusks(root)]  // Root module with 'root' flag
pub mod cli {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub verbose: &'a bool,
    }

    /// Git operations
    #[command(about = "Git commands")]
    pub use crate::git::cli as git;
}
```

**src/git.rs:**
```rust
use tusks::tusks;

#[tusks()]  // External module WITHOUT 'root' flag
pub mod cli {
    // Parent reference is required for parameter chaining
    pub use crate::cli as parent_;

    pub struct Parameters<'a> {
        #[arg(long)]
        pub branch: &'a String,
    }

    /// Commit changes
    pub fn commit(params: &Parameters, message: String) {
        println!("Committing on branch {}: {}", 
                 params.branch, message);
        
        // Access root parameters via super_
        if *params.super_.verbose {
            println!("Verbose output enabled");
        }
    }
}
```

**Usage:**
```bash
$ my-cli --verbose git --branch main commit "Fix bug"
Committing on branch main: Fix bug
Verbose output enabled
```

#### Important Notes on External Modules

- **Parent reference required**: External modules must always contain a `parent_` reference to their parent module. The alias `parent_` must be used.
  ```rust
  pub use crate::cli as parent_;  // Required!
  ```
  This reference also enables parameter chaining via `super_`.

- **No `root` flag**: External modules use `#[tusks()]` **without** the `root` flag. Only the main module (the entry point for the CLI) uses `#[tusks(root)]`.

- **Customize subcommand names**: The name of the subcommand is determined by the name used during import:
  
  ```rust
  // Subcommand is named "cli" (name of imported module)
  pub use crate::git::cli;
  
  // Subcommand is named "git" (with alias)
  pub use crate::git::cli as git;
  
  // Subcommand is named "vcs" (with different alias)
  pub use crate::git::cli as vcs;
  
  // Customize subcommand name via attribute, alias is ignored
  #[command(name = "version-control")]
  pub use crate::git::cli as git;
  ```

- **Arbitrary nesting**: External modules can themselves include external modules:
  
  **src/git.rs:**
  ```rust
  #[tusks()]
  pub mod cli {
      pub use crate::cli as parent_;
      
      // Git includes the advanced module
      #[command(about = "Advanced git operations")]
      pub use crate::git_advanced::cli as advanced;
  }
  ```
  
  **src/git_advanced.rs:**
  ```rust
  #[tusks()]
  pub mod cli {
      // Reference to git module
      pub use crate::git::cli as parent_;
      
      pub fn rebase(/* ... */) { }
  }
  ```
  
  **Invocation:**
  ```bash
  $ my-cli git advanced rebase
  ```

### 6. Return Values and Exit Codes

Commands can return values that are used as exit codes. Allowed return types are `()`, `u8`, and `Option<u8>`. The return value is always returned by `cli::exec_cli()` as `Option<u8>`.

```rust
#[tusks(root)]
pub mod cli {
    pub fn success() {
        println!("Operation completed successfully");
    }
    
    /// Command with u8 return value
    pub fn check_health() -> u8 {
        println!("Running health checks...");
        
        if all_systems_ok() {
            println!("✓ All systems operational");
            0  // Success
        } else {
            println!("✗ System degraded");
            1  // Error
        }
    }
    
    /// Command with Option<u8> return value
    pub fn validate(file: String) -> Option<u8> {
        println!("Validating {}...", file);
        
        match check_file(&file) {
            Ok(_) => {
                println!("✓ Valid");
                Some(0)  // Success
            }
            Err(e) if e.is_warning() => {
                println!("⚠ Warnings found");
                Some(2)  // Warning
            }
            Err(_) => {
                println!("✗ Invalid");
                None
            }
        }
    }
}

fn main() -> std::process::ExitCode {
    // exec_cli() returns the return value of the executed command
    // In this case, explicitly returns 0 if no return value exists
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

**Usage:**
```bash
$ my-cli check-health
Running health checks...
✗ System degraded
$ echo $?
1

$ my-cli validate config.toml
Validating config.toml...
✓ Valid
$ echo $?
0
```

### 7. Various Argument Types

Tusks supports all Clap argument types.

```rust
#[tusks(root)]
pub mod cli {
    /// Complex command with various argument types
    pub fn build(
        #[arg(short, long)]
        target: String,
        
        #[arg(long)]
        features: Vec<String>,
        
        #[arg(long)]
        release: bool,
        
        #[arg(long)]
        jobs: Option<u32>,
    ) {
        println!("Building target: {}", target);
        println!("Features: {:?}", features);
        println!("Release mode: {}", release);
        println!("Jobs: {:?}", jobs.unwrap_or(4));
    }
}
```

**Usage:**
```bash
$ my-cli build --target x86_64-linux --features async --features logging --release --jobs 8
Building target: x86_64-linux
Features: ["async", "logging"]
Release mode: true
Jobs: 8
```

### 8. Custom Value Parsers

Clap's value parsers can be used to add custom validation.

```rust
#[tusks(root)]
pub mod cli {
    #[skip]  // This function is not a command
    pub fn parse_port(s: &str) -> Result<u16, String> {
        s.parse::<u16>()
            .map_err(|_| "Port must be between 0 and 65535".into())
    }

    /// Start server
    pub fn serve(
        #[arg(value_parser = crate::cli::parse_port)]
        port: u16
    ) {
        println!("Starting server on port {}", port);
    }
}
```

**Usage:**
```bash
$ my-cli serve 8080
Starting server on port 8080

$ my-cli serve invalid
error: invalid value 'invalid' for '<PORT>': Port must be between 0 and 65535
```

### 9. Tasks Mode (Ruby Rake-Style)

Tasks mode provides a simplified, flat CLI syntax in the style of Ruby Rake. Instead of nested subcommands, tasks can be invoked with a separator (default `.`).

#### Activation

Tasks mode is simply activated through the `tasks` attribute:

```rust
#[tusks(root, tasks)]
#[command(about = "Task management tool")]
pub mod tasks {
    #[command(about = "Git operations")]
    pub mod git {
        /// Clone a repository
        pub fn clone(url: String, path: Option<String>) {
            println!("Cloning {} to {:?}", url, path);
        }

        /// Commit changes
        pub fn commit(message: String) {
            println!("Committing: {}", message);
        }
    }

    #[command(about = "Docker operations")]
    pub mod docker {
        /// Build Docker image
        pub fn build(context: String, tag: Option<String>) {
            println!("Building from {} with tag {:?}", context, tag);
        }
    }
}
```

#### Display Task List

Without arguments, the CLI automatically shows all available tasks in a grouped overview:

```bash
$ tasks
Task management tool

  git
    git.clone     Clone a repository
    git.commit    Commit changes

  docker
    docker.build  Build Docker image
```

#### Flat Task Syntax

Tasks can be invoked directly via their full path:

```bash
# Rake-style (with separator)
$ tasks git.clone https://github.com/user/repo

# Equivalent to traditional subcommand syntax
$ tasks git clone https://github.com/user/repo
```

Both variants are fully interchangeable. The subcommand structure is preserved, so you can still use module-specific parameters:

```bash
$ tasks --root-param value git --git-option xyz clone https://...
```

#### Help for Tasks

Help can be accessed in multiple ways:

```bash
# With 'h' prefix
$ tasks h git.clone

# With '-h' flag
$ tasks git.clone -h

# Traditional
$ tasks git clone --help
```

All three variants display the same help:
```
Clone a repository

Usage: tasks git clone <URL> [PATH]

Arguments:
  <URL>   Repository URL
  [PATH]  Target path

Options:
  -h, --help  Print help
```

#### Configuring Task Grouping

The task overview display can be configured:

```rust
#[tusks(root, tasks(max_groupsize=5, max_depth=20, separator="."))]
pub mod tasks {
    // ...
}
```

**Parameters:**

- **`separator`** (default: `"."`) - Separator between module and task
  ```bash
  # With separator="."
  $ tasks git.clone url
  
  # With separator=":"
  $ tasks git:clone url
  ```

- **`max_groupsize`** (default: `5`) - Maximum number of tasks per group before further subdivision

  This parameter only affects the task overview output.
  
  The grouping heuristic divides tasks hierarchically:
  1. First level: Grouping by top-level modules (e.g., `git`, `docker`)
  2. If a group contains more than `max_groupsize` tasks, it is further subdivided
  3. Groups with only one element are merged with their parent group

  Example with `max_groupsize=3`:
  ```bash
  # Module 'git' has 2 tasks (≤3) - remains one group
  git
    git.clone
    git.commit
  
  # Module 'docker' has 5 tasks (>3) - is further subdivided
  docker.image
    docker.image.build
    docker.image.push
  docker.container
    docker.container.run
    docker.container.stop
  ```

- **`max_depth`** (default: `20`) - Maximum nesting depth for grouping

  This parameter only affects the task overview output.
  
  Prevents overly deep nesting in the output:
  - Depth 0: Root level (no grouping)
  - Depth 1: Grouping by top-level modules
  - Depth 2: Grouping by submodules
  
  When `max_depth` is reached, no further subdivisions are made, even if `max_groupsize` is exceeded.

**Note:** Currently there are still bugs regarding the `max_groupsize` and `max_depth` parameters. The behavior is not exactly as described above and will likely change in the future.

#### Complete Example

```rust
use tusks::tusks;

#[tusks(root, tasks(separator=".", max_groupsize=5, max_depth=10))]
#[command(
    about = "DevOps toolkit",
    version = "1.0.0"
)]
pub mod tasks {
    pub mod deploy {
        /// Deploy to staging
        pub fn staging(version: String) {
            println!("Deploying {} to staging", version);
        }

        /// Deploy to production
        pub fn production(version: String) {
            println!("Deploying {} to production", version);
        }
    }

    pub mod test {
        /// Run unit tests
        pub fn unit() {
            println!("Running unit tests");
        }

        /// Run integration tests
        pub fn integration() {
            println!("Running integration tests");
        }
    }
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(tasks::exec_cli().unwrap_or(0) as u8)
}
```

**Usage:**
```bash
# Task overview
$ tasks
DevOps toolkit

  deploy
    deploy.staging      Deploy to staging
    deploy.production   Deploy to production

  test
    test.unit          Run unit tests
    test.integration   Run integration tests

# Execute tasks (both syntaxes work)
$ tasks deploy.staging v1.2.3
$ tasks deploy staging v1.2.3

# Display help
$ tasks h deploy.production
$ tasks deploy.production -h
```

### 10. Command Attributes for Documentation

Use `#[command()]` attributes to define CLI documentation. This is a standard Clap feature and is mentioned here only as a common use case.

```rust
#[tusks(root)]
#[command(
    about = "Project management CLI",
    long_about = "A comprehensive tool for managing development projects",
    version = "2.0.0",
    author = "Your Team <team@example.com>"
)]
pub mod cli {
    /// Initialize new project
    #[command(
        about = "Create a new project",
        long_about = "Initialize a new project with default structure and configuration"
    )]
    pub fn init(
        #[arg(help = "Project name")]
        name: String,
        
        #[arg(long, help = "Project template to use")]
        template: Option<String>,
    ) {
        println!("Initializing project: {}", name);
    }
}
```

**Usage:**
```bash
$ my-cli --help
Project management CLI

A comprehensive tool for managing development projects

Usage: my-cli [OPTIONS] <COMMAND>

Commands:
  init  Create a new project
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 11. Default Functions for Modules

With the `#[default]` attribute, you can define a function that is executed when a module is invoked without a specific subcommand.

```rust
#[tusks(root)]
pub mod cli {
    #[command(about = "Git operations")]
    pub mod git {
        pub struct Parameters<'a> {
            #[arg(long)]
            pub repository: &'a Option<String>,
        }
        
        /// Default action when just "git" is called
        #[default]
        pub fn status(params: &Parameters) {
            println!("Git status for {:?}", params.repository);
            // Shows status
        }
        
        /// Push changes
        pub fn push(branch: String) {
            println!("Pushing to {}", branch);
        }
        
        /// Pull changes
        pub fn pull(branch: String) {
            println!("Pulling from {}", branch);
        }
    }
}
```

**Usage:**
```bash
# Without subcommand - executes the default function
$ my-cli git --repository myrepo
Git status for Some("myrepo")

# Equivalent to
$ my-cli git status --repository myrepo
```

#### Restrictions for Default Functions

- **Only Parameters allowed**: Default functions may have at most the `Parameters` struct of the current level as an argument. Additional parameters are not permitted:
  ```rust
  #[default]
  pub fn default_action(params: &Parameters) { }  // ✓ Allowed
  
  #[default]
  pub fn default_action() { }  // ✓ Allowed (without Parameters)
  
  #[default]
  pub fn default_action(params: &Parameters, name: String) { }  // ❌ Not allowed
  ```

- **Exception: Allow External Subcommands**: If `allow_external_subcommands = true` is set for the module, the default function may additionally receive a `Vec<String>` argument containing all arguments of the external subcommand:
  ```rust
  #[command(about = "Command runner", allow_external_subcommands = true)]
  pub mod run {
      pub struct Parameters<'a> {
          #[arg(long)]
          pub verbose: &'a bool,
      }
      
      #[default]
      pub fn execute(params: &Parameters, args: Vec<String>) {
          println!("Running external command with args: {:?}", args);
          if *params.verbose {
              println!("Verbose mode enabled");
          }
          // Executes external command with args
      }
      
      /// Built-in command
      pub fn builtin() {
          println!("This is a built-in command");
      }
  }
  ```
  
  **Usage:**
  ```bash
  # External subcommand (not defined) - default function with args
  $ my-cli run --verbose custom-script --arg1 --arg2
  Running external command with args: ["custom-script", "--arg1", "--arg2"]
  Verbose mode enabled
  
  # Built-in subcommand
  $ my-cli run builtin
  This is a built-in command
  ```

## Comprehensive Example

Here's a complete example demonstrating most of Tusks' features in a single application:

**src/main.rs:**
```rust
use tusks::tusks;

#[tusks(root)]
#[command(
    about = "DevOps automation toolkit",
    long_about = "A comprehensive CLI for managing deployments, databases, and CI/CD pipelines",
    version = "1.0.0",
    author = "DevOps Team <devops@example.com>"
)]
pub mod cli {
    // Top-level command: Quick health check
    /// Perform a quick system health check
    pub fn health() -> u8 {
        println!("Running system health checks...");
        
        // Simulate health check
        let healthy = check_system_health();
        
        if healthy {
            println!("✓ All systems operational");
            0  // Success exit code
        } else {
            println!("✗ System issues detected");
            1  // Error exit code
        }
    }

    // Submodule with multiple commands
    #[command(about = "Database operations")]
    pub mod database {
        // Default command when just "database" is called
        #[default]
        /// Show database status (default action)
        pub fn status(
            #[arg(long, help = "Database connection string")]
            connection: String,
        ) {
            println!("Database status for: {}", connection);
            println!("Connection pool: 10/50");
            println!("Active queries: 3");
        }

        /// Migrate database to latest version
        pub fn migrate(
            #[arg(long, help = "Database connection string")]
            connection: String,
            #[arg(help = "Target migration version")]
            version: String,
            #[arg(long, help = "Perform dry-run without applying changes")]
            dry_run: bool,
        ) -> Option<u8> {
            println!("Migrating database to version: {}", version);
            println!("Connection: {}", connection);
            
            if dry_run {
                println!("⚠ Dry-run mode - no changes applied");
                return Some(0);
            }
            
            // Simulate migration
            let success = perform_migration(&version);
            
            if success {
                println!("✓ Migration completed successfully");
                Some(0)
            } else {
                println!("✗ Migration failed");
                None
            }
        }

        // Nested submodule
        #[command(about = "Advanced database operations")]
        pub mod advanced {
            /// Optimize database tables
            pub fn optimize(
                #[arg(long, help = "Database connection string")]
                connection: String,
            ) -> u8 {
                println!("Optimizing database: {}", connection);
                println!("✓ Optimization completed");
                0
            }
        }
    }

    // External module for deployment operations
    #[command(about = "Deployment commands")]
    pub use crate::deploy::cli as deploy;

    // Helper function with custom value parser
    #[skip]
    pub fn parse_port(s: &str) -> Result<u16, String> {
        let port: u16 = s.parse()
            .map_err(|_| format!("'{}' is not a valid port number", s))?;
        
        if port < 1024 {
            return Err("Port must be 1024 or higher".to_string());
        }
        
        Ok(port)
    }

    // Helper function (not a command)
    #[skip]
    fn check_system_health() -> bool {
        // Simulate health check
        true
    }

    #[skip]
    fn perform_migration(_version: &str) -> bool {
        // Simulate migration
        true
    }
}

fn main() -> std::process::ExitCode {
    // Execute the CLI and use the return value as exit code
    std::process::ExitCode::from(cli::exec_cli().unwrap_or(0) as u8)
}
```

**src/deploy.rs:**
```rust
use tusks::tusks;

#[tusks()]  // External module
pub mod cli {
    // Reference to parent module for parameter chaining
    pub use crate::cli as parent_;

    /// Deploy application to target environment
    pub fn start(
        #[arg(help = "Application version to deploy")]
        version: String,
        #[arg(long, help = "Target environment")]
        environment: String,
        #[arg(long, help = "Server port", value_parser = crate::cli::parse_port)]
        port: Option<u16>,
    ) -> u8 {
        let port = port.unwrap_or(8080);
        
        println!("Deploying version {} to {}", version, environment);
        println!("Server will listen on port: {}", port);
        println!("✓ Deployment initiated");
        0
    }

    /// Rollback to previous version
    pub fn rollback(
        #[arg(long, help = "Target environment")]
        environment: String,
    ) {
        println!("Rolling back deployment in {}", environment);
        println!("✓ Rollback completed");
    }
}
```

**Usage examples:**

```bash
# Top-level command
$ my-cli health
Running system health checks...
✓ All systems operational

# Default command (status is called automatically)
$ my-cli database --connection "postgres://localhost/mydb"
Database status for: postgres://localhost/mydb
Connection pool: 10/50
Active queries: 3

# Explicit subcommand
$ my-cli database status --connection "postgres://localhost/mydb"
Database status for: postgres://localhost/mydb
Connection pool: 10/50
Active queries: 3

# Migration with various argument types
$ my-cli database migrate --connection "postgres://localhost/mydb" v2.0 --dry-run
Migrating database to version: v2.0
Connection: postgres://localhost/mydb
⚠ Dry-run mode - no changes applied

# Nested submodule command
$ my-cli database advanced optimize --connection "postgres://localhost/mydb"
Optimizing database: postgres://localhost/mydb
✓ Optimization completed

# External module with custom value parser
$ my-cli deploy start v1.5.0 --environment production --port 8080
Deploying version v1.5.0 to production
Server will listen on port: 8080
✓ Deployment initiated

# Custom parser validation
$ my-cli deploy start v1.4.0 --environment staging --port 80
error: invalid value '80' for '--port <PORT>': Port must be 1024 or higher

# Rollback command
$ my-cli deploy rollback --environment production
Rolling back deployment in production
✓ Rollback completed
```

**Note:** This example demonstrates how arguments can be defined directly in function signatures. You can also define module-level parameters using a `Parameters` struct which would then be available to all commands within that module and automatically passed down to nested submodules. The parameters would be specified before the subcommand name:

```bash
# Example if database module had Parameters with --connection and --verbose:
$ my-cli database --connection "..." --verbose migrate v2.0
$ my-cli database --connection "..." status
```

For more details on how to define and use module-level parameters, see [Module-Level Parameters](#4-module-level-parameters).

### Equivalent with Raw Clap Syntax

Here's what the same CLI structure would look like using Clap's derive API directly (abbreviated for brevity):

```rust
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(
    about = "DevOps automation toolkit",
    long_about = "A comprehensive CLI for managing deployments, databases, and CI/CD pipelines",
    version = "1.0.0",
    author = "DevOps Team <devops@example.com>"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform a quick system health check
    Health,
    
    /// Database operations
    Database {
        #[command(subcommand)]
        command: Option<DatabaseCommands>,
    },
    
    /// Deployment commands
    Deploy {
        #[command(subcommand)]
        command: DeployCommands,
    },
}

#[derive(Subcommand)]
enum DatabaseCommands {
    /// Show database status
    Status {
        #[arg(long, help = "Database connection string")]
        connection: String,
    },
    
    /// Migrate database to latest version
    Migrate {
        #[arg(long, help = "Database connection string")]
        connection: String,
        version: String,
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Advanced database operations
    Advanced {
        #[command(subcommand)]
        command: AdvancedCommands,
    },
}

#[derive(Subcommand)]
enum AdvancedCommands {
    /// Optimize database tables
    Optimize {
        #[arg(long, help = "Database connection string")]
        connection: String,
    },
}

#[derive(Subcommand)]
enum DeployCommands {
    /// Deploy application
    Start {
        version: String,
        #[arg(long, help = "Target environment")]
        environment: String,
        #[arg(long, value_parser = parse_port)]
        port: Option<u16>,
    },
    
    /// Rollback to previous version
    Rollback {
        #[arg(long, help = "Target environment")]
        environment: String,
    },
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    
    // Manual dispatch logic
    let exit_code = match cli.command {
        Some(Commands::Health) => {
            health()
        }
        Some(Commands::Database { command }) => {
            match command {
                Some(DatabaseCommands::Status { connection }) | None => {
                    // Default command logic needed here
                    database_status(&connection);
                    0
                }
                Some(DatabaseCommands::Migrate { connection, version, dry_run }) => {
                    database_migrate(&connection, &version, dry_run)
                        .unwrap_or(1)
                }
                Some(DatabaseCommands::Advanced { command }) => {
                    match command {
                        AdvancedCommands::Optimize { connection } => {
                            database_advanced_optimize(&connection)
                        }
                    }
                }
            }
        }
        Some(Commands::Deploy { command }) => {
            match command {
                DeployCommands::Start { version, environment, port } => {
                    deploy_start(&version, &environment, port)
                }
                DeployCommands::Rollback { environment } => {
                    deploy_rollback(&environment);
                    0
                }
            }
        }
        None => {
            // Show help when no command is provided
            Cli::parse_from(&["cli", "--help"]);
            0
        }
    };
    
    std::process::ExitCode::from(exit_code)
}

// All the handler functions need to be manually implemented
fn health() -> u8 {
    // Implementation
    0
}

fn database_status(connection: &str) {
    // Implementation
}

fn database_migrate(connection: &str, version: &str, dry_run: bool) -> Option<u8> {
    // Implementation
    Some(0)
}

fn database_advanced_optimize(connection: &str) -> u8 {
    // Implementation
    0
}

fn deploy_start(version: &str, environment: &str, port: Option<u16>) -> u8 {
    // Implementation
    0
}

fn deploy_rollback(environment: &str) {
    // Implementation
}
```

As you can see, Tusks eliminates:
- Manual enum definitions for every command level
- Repetitive match/dispatch logic
- Manual parameter passing through the hierarchy
- Boilerplate for default command handling

## License

MIT

## Contributions

Contributions are welcome! Please create an issue or pull request on GitHub.

I'm still learning Rust. I always have an open ear for suggestions on best practices, code style, etc.

I will try to respond to contributions, but as I work full-time with a wife and child, this will not always be possible in a timely manner.

## Trivia

This project originally came about because I've wanted to learn and use Rust for a long time, and also because I was looking for a replacement for Ruby Rake and Python Invoke that is idiomatic, future-oriented, and easy to use. The use of these tools is also always limited by the environment you're in. Often, the versions of interpreters and packages differ across various server environments. A compiled Rust application is much more flexible in this regard. However, there are also disadvantages. The effort and barrier to creating and extending a compiled application is higher than with a simple Python script. And of course, the appropriate toolchain must be available on the system where you develop. But Rust makes it quite easy with cargo.

During development, I worked extensively with various AIs. Otherwise, this would not have been possible within a week with my existing basic knowledge, especially not for a project that relies so heavily on source code parsing and generation, i.e., the creation of macros. In some places, I performed some refactoring. In other places, I only superficially adapted or reviewed the code. Little code was actually written 100% by myself. This is an interesting experience for me, and you can certainly view it critically. However, I think I still learned a lot, and I'm actually quite satisfied with the code quality. If I had actually done everything myself, it would probably have turned out significantly worse (or simply not finished). But one should always keep in mind that I'm a Rust beginner. The way to write code in Rust, to structure it, the patterns used - all of this is definitely very different in many ways from many programming languages I've used before. Accordingly, this is a beginner's project.
