pub mod cfg;
pub mod integration;

pub fn explain() {
    //There are two types of tests: 
        //unit tests (test a module in isolation, can access private vars) 
        //and integration tests (external to code, may access multiple mods)
    println!("Use both unit tests and integration tests to ensure your code works.");
    cfg::explain();
}   

pub fn r#return() {
    println!("Apparently there's a debate that private functions shouldn't be able to be tested. Weird.");
    //In rust you can test them tho
    //Remember child mods can access parents' private stuff
    integration::explain();
}

fn this_is_private(num: &mut i32) {
    *num += 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_but_testable() {
        let mut num = 20;
        this_is_private(&mut num);
        assert_eq!(30, num, "Failed, num was {num} (expected 30)");
    }
}