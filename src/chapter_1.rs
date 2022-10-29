// https://learnku.com/docs/nomicon/2018/21-repr/4708
// https://nomicon.purewhite.io/repr-rust.html
// https://doc.rust-lang.org/nomicon/exotic-sizes.html

// 2. Data Layout - repr(Rust)

use std::mem;

struct A {
    a: u8,
    b: u32,
    c: u16,
}

struct B {
    a: u8,
    b: u32,
    c: u16,
}

pub fn repr() {
    let a = A { a: 1, b: 1, c: 1 };
    println!("struct A size = {}", mem::size_of::<A>());
    println!("struct B size = {}", mem::size_of::<B>());
    println!("size of variable a = {}", mem::size_of_val(&a));
}