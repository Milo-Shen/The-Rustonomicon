// https://nomicon.purewhite.io/other-reprs.html
// https://doc.rust-lang.org/nomicon/other-reprs.html

// 可选的数据布局
// Rust 允许你指定不同于默认的数据布局策略，并为你提供了不安全代码指南。
// repr(c): 数据的顺序、大小、对齐方式都和 C / C++ 中一样
//  1. 但是 ZST 类型大小仍然是 0
//  2. DST 类型不是 FFI 安全的
//  3. 如果 T 是一个 FFI 安全的类型, 那么 Option<T> 和 T 拥有同样的布局, 也是安全的
//  4. repr(C) 和 repr(u*) 中无成员的枚举不能被赋值为一个没有对应变量的整数, 但是 C \ C++ 中可以

// repr(transparent): 这只能用于具有某个非零大小字段 ( 可能还有其他零大小字段 ) 的结构。其效果是保证整个结构的布局和 ABI 与该字段相同
// repr(u*) 或 repr(i*)
// repr(packed)
