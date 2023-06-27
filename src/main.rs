#![allow(unused, dead_code)]

pub mod writing;
pub mod controlling;
pub mod organizing;

fn main() {
    topic::introduce();
    writing::explain();
    controlling::explain();
    organizing::explain();
}

pub mod topic {
    pub fn introduce() {
        println!("
        ------------------------CHAPTER 11: HOW TO WRITE TESTS------------------------
        Rust has built-in test support so your code can be as correct as possible.
        ");
    }
}
