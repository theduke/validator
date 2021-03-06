#![feature(attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
struct Test {
    #[validate(range(min = 1, max = 2.2))]
    s: isize,
    #[validate(range(min = 1, max = 2))]
    s2: usize,
    #[validate(range(min = 18, max = 22))]
    s3: i32,
    #[validate(range(min = 18, max = 22))]
    s4: i64,
    #[validate(range(min = 18, max = 22))]
    s5: u32,
    #[validate(range(min = 18, max = 22))]
    s6: u64,
    #[validate(range(min = 18.1, max = 22))]
    s7: i8,
    #[validate(range(min = 18.0, max = 22))]
    s8: u8,
}

fn main() {}
