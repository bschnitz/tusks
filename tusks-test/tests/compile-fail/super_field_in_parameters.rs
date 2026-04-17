use tusks::tusks;

#[tusks(root)]
pub mod cli {
    pub struct Parameters<'a> {
        pub super_: &'a str,
    }

    pub fn hello() {
        println!("hello");
    }
}

fn main() {}
