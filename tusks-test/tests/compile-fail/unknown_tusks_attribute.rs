use tusks::tusks;

#[tusks(root, nonexistent)]
pub mod cli {
    pub fn hello() {
        println!("hello");
    }
}

fn main() {}
