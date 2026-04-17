use tusks::tusks;

#[tusks(root)]
pub mod cli {
    pub struct Parameters {
        pub name: String,
    }

    pub fn hello() {
        println!("hello");
    }
}

fn main() {}
