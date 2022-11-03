// https://learnku.com/docs/nomicon/2018/21-repr/4708
// https://nomicon.purewhite.io/repr-rust.html
// https://doc.rust-lang.org/nomicon/exotic-sizes.html

// 2. Data Layout - repr(Rust)

// 对其
// 1. 所有类型都有一个以字节为单位指定的对齐方式
// 2. 在 x86 平台上 u64 和 f64 都是按照 32 位对其的
// 3. 一种类型的大小是它对齐属性的整数倍
// 4. 动态尺寸类型的大小和对齐无法静态获取

// Rust 复合类型
// 结构体 ( 带命名的复合类型 )
// 元组 ( 匿名的复合类型 )
// 数组 ( 同类型数据集合 )
// 枚举 ( 带命名的标签联合体 )

// 结构体
// 结构体的对齐等于它所有成员的对齐属性中最大的那个。
// Rust 会在必要的位置填充空白数据, 以保证每一个成员都能正确地对齐
// 同时整个类型的尺寸是对齐属性的整数倍

// 除数组外 ( 数组的子类型总是按照顺序紧密排列 ), 其他复合类型的数据分布规则不一定是固定不变的
// Rust 会优化布局规则

// 枚举
// 大部分枚举类型的布局如下面的例子:

enum Foo {
    A(u32),
    B(u64),
    C(u8),
}

// // 布局为:
// struct FooRepr{
//     data:u64, // 根据 tag 的不同，这一项可以为 u64, u32, u8 中的一项
//     tag:u8 // 0 = A, 1 = B, 2 = C
// }

// 特例:
// 如果一个枚举类型只包含一个单值变量 ( 比如 None ) 和一个级联的非 null 指针变量 ( 比如 &T ),
// 那么 tag 其实是不需要的，因为那个单值变量完全可以用 null 指针来表示

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
    println!("enum Foo size = {}", mem::size_of::<Foo>());
    println!("enum &Foo size = {}", mem::size_of::<&Foo>());
}
