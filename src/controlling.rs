pub fn explain() {
    println!("Here's a quick guide for how you can control what tests are run:
        To provide a test to the binary, use
        To not make tests hide println! msgs and all other output display, use -- --show-output
        To control how many threads the tests are run on, use -- --test-threads=(insert num)
        To run one specific test, specify its name w/o dashes
        To only run tests that have a specific substring in them, specify it w/o dashes
        To ignore a test, use #[ignore]
        To run only ignored tests, use -- --ignored, and to run all tests use -- --include-ignored
    ");
}