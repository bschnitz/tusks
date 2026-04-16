use tusks::tusks;

#[tusks()]
#[command(
    about = "External module 2 - maximum nesting",
    long_about = "Demonstrates four-level parameter chain: ext2 -> ext1 -> level1 -> root"
)]
pub mod tasks {
    pub use crate::external1::tasks as parent_;

    pub struct Parameters<'a> {
        #[arg(long)]
        pub ext2_param: &'a String,
    }

    /// Deeply nested external task - demonstrates deep super_ chain
    #[command(
        about = "Deep external task",
        long_about = "Access parameters from 4 levels: ext2 -> ext1 -> level1 -> root"
    )]
    pub fn ext2_task(
        params: &Parameters,
        #[arg(short, long)]
        x: String
    ) -> u8 {
        println!("=== external2::ext2_task ===");
        println!("  ext2_param: {}", params.ext2_param);
        println!("  x: {}", x);
        
        // Access ext1 parameters via parent_
        println!("  ext1_param (via parent_): {:?}", params.super_.ext1_param);
        
        // Access level1 parameters via parent_.super_
        println!("  level1_field (via parent_.super_): {:?}", 
                 params.super_.super_.level1_field);
        
        // Access root parameters via parent_.super_.super_
        println!("  root_param (via parent_.super_.super_): {}", 
                 params.super_.super_.super_.root_param.as_ref().unwrap());
        
        println!("  Parameter chain: ext2 -> ext1 -> level1 -> root");
        println!("    Depth: 4 levels!");
        
        if *params.super_.super_.super_.verbose {
            println!("  [VERBOSE] ext2 accessing root 3 levels up!");
        }
        
        0
    }

    /// Task with multiple types - demonstrates complex parameter access
    #[command(
        about = "Complex task with multiple argument types",
        long_about = "Tests bool flags, Vec parameters, and Option types together"
    )]
    pub fn ext2_complex(
        params: &Parameters,
        #[arg(long)]
        flag: bool,
        #[arg(long)]
        numbers: Vec<u8>,
        #[arg(short)]
        optional: Option<String>
    ) -> u8 {
        println!("=== external2::ext2_complex ===");
        println!("  ext2_param: {}", params.ext2_param);
        println!("  flag: {}", flag);
        println!("  numbers: {:?}", numbers);
        println!("  optional: {:?}", optional);
        
        // Show the full parameter chain
        println!("\n  Full parameter chain access:");
        println!("    ext2_param: {}", params.ext2_param);
        println!("    ext1_param: {:?}", params.super_.ext1_param);
        println!("    level1_field: {:?}", params.super_.super_.level1_field);
        println!("    root_param: {}", params.super_.super_.super_.root_param.as_ref().unwrap());
        
        42
    }
}
