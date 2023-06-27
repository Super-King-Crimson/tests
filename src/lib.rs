#![allow(unused, dead_code)]

use std::process::{Termination, ExitCode};
pub mod writing;
pub mod controlling;
pub mod organizing;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Rectangle {
    length: u32,
    width: u32,
}

//don't ask.
impl Termination for Rectangle {
    fn report(self) -> ExitCode {
        ExitCode::from(255)
    }
}

impl Rectangle {
    pub fn new(length: u32, width: u32) -> Rectangle {
        if u32::MAX / length < width {
            panic!("Cannot make a rectangle with area larger than u32::MAX ({})", u32::MAX);
        } else {
            Rectangle { length, width }
        }
    }

    pub fn area(&self) -> u32 {
        self.length * self.width
    }

    //A bug! Perimeter is 2(l + w), not 2l + w!
    pub fn perimeter(&self) -> u32 {
        2 * self.length + self.width
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn biggest_perimeter(this: Rectangle, that: Rectangle) -> Rectangle {
        if this.perimeter() > that.perimeter() {this} else {that} 
    }

    pub fn combine_and_report(&mut self, other: Rectangle) -> String {
        self.length += other.length;
        self.width += other.width;


        format!("My new length is {}, my new width is {}", self.width, self.length) //another bug!
    }
}

//this attribute tells Rust to only compile the tests when you run cargo test
#[cfg(test)]
mod tests {
    use super::*; //so we can use the Rectangle struct we defined

    #[test] //hey, it's the test attribute! now the compiler knows it_works is a test fn
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); //panics if result doesn't equal 4
    }
    //Tests can be filtered (only run if name matches a string) and ignored (only run if specifically asked for)
    //They can also be benchmarked, which can only be done on nightly Rust (check the docs for info on that) 
    //You can run the tests with cargo test
    //When you do this, you'll see something called Doc-tests (ignore them they're documentation tests)

    #[test]
    fn it_doesnt_work() {
        assert_eq!(add(1, 3), 5, "Not equal");
    }

    #[test]
    fn can_hold() {
        let big = Rectangle { length: 20, width: 15 };
        let small = Rectangle { length: 6, width: 1};

        let long = Rectangle { length: 50, width: 1 };
        let wide = Rectangle { length: 1, width: 25 };

        assert!(big.can_hold(&small));
        assert!(!long.can_hold(&small));
    }

    #[test]
    fn area() {
        let big = Rectangle { length: 20, width: 15 };
        let small = Rectangle { length: 5, width: 5 };

        let long = Rectangle { length: 50, width: 1 };
        let wide = Rectangle { length: 1, width: 25 };

        assert_eq!(small.area(), wide.area());
        assert_ne!(long.area(), big.area());
    }

    #[test]
    fn biggest_perimeter() {
        let big = Rectangle { length: 20, width: 15 };
        let small = Rectangle { length: 5, width: 5 };

        let long = Rectangle { length: 25, width: 1 };
        let wide = Rectangle { length: 1, width: 40 };

        //We have to check area because Rectangle doesn't implement PartialEq and Debug 
        //We could also just do #[derive(PartialEq, Debug)] above it but who cares
        assert_eq!(big.area(), Rectangle::biggest_perimeter(big, small).area());
        assert_eq!(wide.area(), Rectangle::biggest_perimeter(long, wide).area());
    }

    #[test]
    fn combine_and_report() {
        let mut big = Rectangle { length: 20, width: 15 };
        let mut small = Rectangle { length: 5, width: 5 };

        let report = big.combine_and_report(small);

        //Check that report contains the other arg (passed to format, can use other args)
        assert!(report.contains("length is 25"), "Did not contain correct length, report was '{}'", report);
        assert!(report.contains("width is 20"), "Did not contain correct width, report was '{}'", report);
    }

    #[test]
    #[should_panic(expected = "MAX")]
    fn new() {
        let okay = Rectangle::new(10, 30);
        // panic!("Wrong error message, doesn't contain 'max' in all caps!");
        let not_okay = Rectangle::new(100000, u32::MAX);
    }

    #[test]
    fn result() -> Result<Rectangle, String> {
        let rect = Rectangle::new(10, 20);
        let literal_rect = Rectangle { length: 10, width: 290 }; //uh oh
        
        if rect.area() == literal_rect.area() && rect.perimeter() == literal_rect.perimeter() {
            Ok(rect)
        } else {
            Err("The rects are not the same".to_string())
        }
    }
}

