use tusks::tusks;

#[tusks()]
#[command(about = "External module 1 - nested under level1")]
pub mod tasks {
    pub use crate::tasks::level1 as parent_;

    pub struct Parameters<'a> {
        #[arg(long)]
        pub ext1_param: &'a Option<u32>,
    }

    /// External1 task accessing ext1, level1, and root parameters
    #[command(
        about = "External1 main task",
        long_about = "Demonstrates three-level parameter chain: ext1 -> level1 -> root"
    )]
    pub fn ext1_task(params: &Parameters) {
        println!("=== external1::ext1_task ===");
        println!("  ext1_param: {:?}", params.ext1_param);
        
        // Access level1 parameters via parent_
        println!("  level1_field (via parent_): {:?}", params.super_.level1_field);
        println!("  level1_number (via parent_): {}", params.super_.level1_number);
        
        // Access root parameters via parent_.super_
        println!("  root_param (via parent_.super_): {}", params.super_.super_.root_param.as_ref().unwrap());
        println!("  verbose (via parent_.super_): {}", params.super_.super_.verbose);
        
        if *params.super_.super_.verbose {
            println!("  [VERBOSE] external1 can access root through level1!");
        }
    }

    /// External1 task with Vec argument
    #[command(about = "Process vector of integers")]
    pub fn ext1_vec_task(
        params: &Parameters,
        #[arg(long)]
        values: Vec<u8>
    ) -> Option<u8> {
        println!("=== external1::ext1_vec_task ===");
        println!("  ext1_param: {:?}", params.ext1_param);
        println!("  values: {:?}", values);
        let sum = values.iter().sum::<u8>();
        println!("  sum: {}", sum);
        
        // Demonstrate super_ chain
        println!("  Chain to root: ext1 -> level1 -> root");
        println!("    root_param: {}", params.super_.super_.root_param.as_ref().unwrap());
        
        Some(sum as u8)
    }

    // Nested external module
    #[command(about = "External module 2 - deeply nested")]
    pub use crate::external2::tasks as ext2;
}
