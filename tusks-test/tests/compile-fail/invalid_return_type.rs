use tusks::tusks;

#[tusks(root)]
pub mod cli {
    pub fn hello() -> String {
        "hello".to_string()
    }
}

fn main() {}
