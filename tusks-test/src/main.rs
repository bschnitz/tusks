use tusks::tusks;
mod external_root;
mod external1;
mod external2;

#[tusks(root)]
#[command(
    arg_required_else_help = true,
    about = "Comprehensive CLI testing tool",
    long_about = "A comprehensive command-line interface for testing the tusks macro system with nested modules and parameter chains",
    version = "1.0.0",
    author = "Test Author"
)]
pub mod tasks {
    pub struct Parameters<'a> {
        #[arg(long)]
        pub root_param: &'a Option<String>,
        
        #[arg(short, long)]
        pub verbose: &'a bool,
    }

    #[command(name = "t", allow_external_subcommands = true)]
    pub fn tasks_mode (
        #[arg(trailing_var_arg = true)]
        args: Vec<String>
    ) {
        let command = __internal_tusks_module::cli::Cli::command();
        if args.len() == 0 {
            let task_list = tusks_tasks::task_list::models::TaskList::from_command(
                &command,
                ".".to_string(),
                5,
                20 
            );
            task_list.to_list().print(&tusks_tasks::list::models::RenderConfig::default());
        }
        else {
            let cli = __internal_tusks_module::cli::Cli::parse_from(
                std::iter::once("tusks").chain(args.iter().map(String::as_str))
            );
            __internal_tusks_module::handle_matches(&cli);
        }
    }

    #[skip]
    pub fn parse_port(s: &str) -> Result<u16, String> {
        s.parse::<u16>()
            .map_err(|_| "Port must be an integer".into())
    }

    pub fn task0(
        #[arg(value_parser = crate::tasks::parse_port)]
        port: u16
    ) {
        println!("Port is: {}", port)
    }

    /// Task with Parameters argument - accesses root parameters
    #[command(
        about = "First task with parameters",
        long_about = "Demonstrates accessing root parameters and verbose flag"
    )]
    pub fn task1(params: &Parameters, #[arg(long)] arg1: String) -> u8 {
        println!("=== root::task1 ===");
        println!("  root_param: {}", params.root_param.as_ref().unwrap());
        println!("  verbose: {}", params.verbose);
        println!("  arg1: {}", arg1);
        
        if *params.verbose {
            println!("  [VERBOSE] Executing task1 with root_param='{}'", params.root_param.as_ref().unwrap());
        }
        0
    }

    /// Task without Parameters argument, with return value
    #[command(about = "Doubles a numeric value")]
    pub fn task2(#[arg(short, long)] value: i32) -> u8 {
        println!("=== root::task2 ===");
        println!("  value: {}", value);
        println!("  returning: {}", value * 2);
(value * 2) as u8
    }

    /// Task with no arguments at all
    #[command(about = "Task with no arguments")]
    pub fn task3() {
        println!("=== root::task3 ===");
        println!("  No arguments, just executing");
    }

    /// Task with Vec parameter
    #[command(
        about = "Process a list of items",
        long_about = "Takes multiple items as input and displays them"
    )]
    pub fn task4(
        params: &Parameters,
        #[arg(long)]
        items: Vec<String>
    ) {
        println!("=== root::task4 ===");
        println!("  root_param: {}", params.root_param.as_ref().unwrap());
        println!("  items: {:?}", items);
        println!("  item count: {}", items.len());
    }

    #[command(
        about = "Level 1 submodule",
        long_about = "First level of nested commands with its own parameters"
    )]
    pub mod level1 {
        pub struct Parameters<'a> {
            #[arg(long)]
            pub level1_field: &'a Option<String>,
            
            #[arg(long, default_value = "42")]
            pub level1_number: &'a i32,
        }

        /// Subtask accessing both level1 and root parameters
        #[command(about = "First subtask with parameter chain")]
        pub fn subtask1(
            params: &Parameters,
            #[arg(long)]
            arg: Option<String>
        ) -> u8 {
            println!("=== level1::subtask1 ===");
            println!("  level1_field: {:?}", params.level1_field);
            println!("  level1_number: {}", params.level1_number);
            println!("  arg: {:?}", arg);
            
            // Access parent parameters via super_
            println!("  root_param (via super_): {}", params.super_.root_param.as_ref().unwrap());
            println!("  verbose (via super_): {}", params.super_.verbose);
            
            if *params.super_.verbose {
                println!("  [VERBOSE] Level1 subtask1 accessing root_param='{}'", 
                         params.super_.root_param.as_ref().unwrap());
            }
            1
        }

        /// Subtask with only Parameters argument - demonstrates super_ access
        #[command(
            about = "Second subtask",
            long_about = "Demonstrates accessing parent parameters through super_"
        )]
        pub fn subtask2(params: &Parameters) {
            println!("=== level1::subtask2 ===");
            println!("  level1_field: {:?}", params.level1_field);
            println!("  Accessing root via super_:");
            println!("    root_param: {}", params.super_.root_param.as_ref().unwrap());
            println!("    verbose: {}", params.super_.verbose);
        }

        #[command(about = "Level 2 - deeply nested module")]
        pub mod level2 {
            pub struct Parameters<'a> {
                #[arg(long)]
                pub level2_id: &'a u64,
            }

            /// Deep task accessing level2, level1, and root parameters
            #[command(
                about = "Deep nested task",
                long_about = "Demonstrates accessing parameters from multiple levels up the chain"
            )]
            pub fn deep_task(
                params: &Parameters,
                #[arg(long)] 
                enabled: bool
            ) {
                println!("=== level2::deep_task ===");
                println!("  level2_id: {}", params.level2_id);
                println!("  enabled: {}", enabled);
                
                // Access level1 parameters
                println!("  level1_field (via super_): {:?}", params.super_.level1_field);
                println!("  level1_number (via super_): {}", params.super_.level1_number);
                
                // Access root parameters via super_.super_
                println!("  root_param (via super_.super_): {}", params.super_.super_.root_param.as_ref().unwrap());
                println!("  verbose (via super_.super_): {}", params.super_.super_.verbose);
                
                if *params.super_.super_.verbose {
                    println!("  [VERBOSE] Deep in level2, can still access root!");
                }
            }

            #[command(about = "Maximum depth level")]
            pub mod level3 {
                /// Very deep task - no Parameters, but we could pass parent params
                #[command(about = "Task at maximum nesting depth")]
                pub fn very_deep(#[arg(long)] depth: u32) -> u8 {
                    println!("=== level3::very_deep ===");
                    println!("  depth: {}", depth);
                    println!("  Nested {} levels deep!", depth);
                    depth as u8
                }
            }
        }

        // External module at level1
        #[command()]
        pub use crate::external1::tasks as ext1;
    }

    #[command(about = "Alternative level 1 module without Parameters")]
    pub mod level1b {
        // No Parameters struct in this module

        #[command(about = "Task without module Parameters struct")]
        pub fn task_no_params(#[arg(long)] x: u8) -> u8 {
            println!("=== level1b::task_no_params ===");
            println!("  x: {}", x);
            println!("  No Parameters struct in this module");
            x as u8
        }

        #[command(
            about = "Task with multiple argument types",
            long_about = "Demonstrates various argument types: String, Option, bool, Vec"
        )]
        pub fn task_multi_args(
            #[arg(short, long)]
            name: String,
            #[arg(long)]
            age: Option<i32>,
            #[arg(short)]
            active: bool,
            tags: Vec<String>
        ) {
            println!("=== level1b::task_multi_args ===");
            println!("  name: {}", name);
            println!("  age: {:?}", age);
            println!("  active: {}", active);
            println!("  tags: {:?}", tags);
        }
    }

    pub mod empty_module {
        // Module with only Parameters, no tasks
        pub struct Parameters<'a> {
            #[arg(long)]
            pub empty_field: &'a String,
        }
    }

    // External module at root level
    #[command(
        about = "External root module",
        long_about = "External module attached at root level demonstrating parent_ references"
    )]
    pub use crate::external_root::tasks as extroot;
}

fn main() -> std::process::ExitCode {
    std::process::ExitCode::from(tasks::exec_cli().unwrap_or(0) as u8)
}
