use tusks::tusks;

#[tusks(root)]
pub mod cli {
    #[default]
    pub fn first() {
        println!("first");
    }

    #[default]
    pub fn second() {
        println!("second");
    }
}

fn main() {}
