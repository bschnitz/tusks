use tusks::tusks;

#[tusks()]
pub mod cli {
    pub use crate::cli as parent_;

    pub async fn start(version: String) -> u8 {
        println!("Deploying {}", version);
        0
    }
}
