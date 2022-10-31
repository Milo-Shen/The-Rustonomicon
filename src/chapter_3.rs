// https://nomicon.purewhite.io/other-reprs.html
// https://doc.rust-lang.org/nomicon/other-reprs.html

// 可选的数据布局
// Rust 允许你指定不同于默认的数据布局策略，并为你提供了不安全代码指南。
// repr(c): 数据的顺序、大小、对齐方式都和 C / C++ 中一样
//  1. 但是 ZST 类型大小仍然是 0
//  2. DST 类型不是 FFI 安全的
