

extern crate generic_array;
extern crate object_pool;
extern crate rand;

extern crate core;

pub mod common;

pub mod my;

pub mod full_test;


fn main(){
    full_test::full_test(10)
}
