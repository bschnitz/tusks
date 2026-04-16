use tusks::tusks;

#[tusks()]
#[command(
    about = "External root tasks module",
    long_about = "Demonstrates external module with parent reference to root"
)]
pub mod tasks {
    pub use crate::tasks as parent_;

    pub struct Parameters<'a> {
        #[arg(long)]
        pub extroot_name: &'a String,
    }

    /// External root task accessing both external and root parameters
    #[command(about = "Main external root task")]
    pub fn ext_task(
        params: &Parameters,
        #[arg(long)]
        count: u8
    ) -> u8 {
        println!("=== external_root::ext_task ===");
        println!("  extroot_name: {}", params.extroot_name);
        println!("  count: {}", count);
        
        // Access parent (root) parameters
        println!("  root_param (via parent_): {}", params.super_.root_param.as_ref().unwrap());
        println!("  verbose (via parent_): {}", params.super_.verbose);
        
        if *params.super_.verbose {
            println!("  [VERBOSE] External module accessing root parameters!");
        }
        
        count
    }

    #[command(about = "Submodule in external module")]
    pub mod sub {
        /// Submodule task in external module
        #[command(about = "Task within external submodule")]
        pub fn ext_sub_task(#[arg(long)] msg: String) {
            println!("=== external_root::sub::ext_sub_task ===");
            println!("  msg: {}", msg);
        }
    }
}
