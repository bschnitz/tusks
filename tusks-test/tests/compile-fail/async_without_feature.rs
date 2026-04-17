use tusks::tusks;

#[tusks(root)]
pub mod cli {
    pub async fn hello() {
        println!("hello");
    }
}

fn main() {}
