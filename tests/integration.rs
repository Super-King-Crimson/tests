//We don't need #[cfg(test)] here!

use tests::organizing::integration::*;
mod different_integration_test;
mod helper;

#[test]
fn bad_entry() {
    let mut turnstile = Turnstile::new(5.50);

    let (can_enter, msg) = turnstile.enter();
    assert!(!can_enter);
    assert!(msg.contains("pay"));
}

#[test]
fn paid_entry_cash() {
    let mut turnstile = Turnstile::new(5.50);
    assert_eq!(5.50, turnstile.get_money_to_open());

    let (can_enter, msg) = turnstile.pay(3.50);
    assert!(!can_enter);
    assert!( msg.contains("money") && (msg.contains("not") || msg.contains("n't")), "msg was {msg}" );

    let (can_enter, msg) = turnstile.pay(15.0);
    assert!(can_enter);
    assert!(msg.contains("$9.50"), "msg was {msg}"); //when paying with change it should mention 'your change is XXX'

    let (entered, _) = turnstile.enter();
    assert!(entered);

    let (entered, _) = turnstile.enter();
    assert!(!entered);
}

//Run all tests in an integration test file with cargo test --test file-name
//for this file its cargo test --test integration

#[test]
fn two_is_two() {
    let two = different_integration_test::get_two();
    assert_eq!(two, 2);
}

#[test]
fn now_im_confused() {
    let mut ten = 10;
    helper::wait_srs(&mut ten);
    assert_eq!(ten, 20);
    //ok i thought i forgot how pointers worked for a second
}

//Note that you can't have integration test if you're only a binary crate
