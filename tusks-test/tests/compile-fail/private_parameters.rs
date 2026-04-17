use tusks::tusks;

#[tusks(root)]
pub mod cli {
    struct Parameters<'a> {
        pub name: &'a str,
    }

    pub fn hello() {
        println!("hello");
    }
}

fn main() {}
