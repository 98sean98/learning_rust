use adder;

mod common;

#[test]
fn it_adds() {
    assert_eq!(adder::add(2, 2), 4);
}

#[test]
fn it_adds_v2() {
    common::setup();

    assert_eq!(adder::add(3, 4), 7);
}