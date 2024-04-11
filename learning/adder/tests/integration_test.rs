use adder;

// These are integration tests
// tested from a black-box perspective

// You can test this file specifically
// using --test integration_test

// Import common code
mod common;

// Note: binary crates are not able to be
// tested this way, since only library crates
// expose their functions.
// To make integration tests for binary crates,
// provide a src/main.rs that calls logic that lives 
// in src/lib.rs
// You can then use 'use' to make the functionality
// available
// As of the writing, I have no idea what that means.
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}