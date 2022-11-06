// https://nomicon.purewhite.io/other-reprs.html
// https://doc.rust-lang.org/nomicon/other-reprs.html

use std::mem;

// 可选的数据布局
// Rust 允许你指定不同于默认的数据布局策略，并为你提供了不安全代码指南。
// repr(c): 数据的顺序、大小、对齐方式都和 C / C++ 中一样
//  1. 但是 ZST 类型大小仍然是 0
//  2. DST 类型不是 FFI 安全的
//  3. 如果 T 是一个 FFI 安全的类型, 那么 Option<T> 和 T 拥有同样的布局, 也是安全的
//  4. repr(C) 和 repr(u*) 中无成员的枚举不能被赋值为一个没有对应变量的整数, 但是 C\C++ 中可以

// repr(transparent): 这只能用于具有某个非零大小字段 ( 可能还有其他零大小字段 ) 的结构。其效果是保证整个结构的布局和 ABI 与该字段相同
// repr(u*) 或 repr(i*)
// repr(packed)

// 这是最重要的 “repr”。它的意图相当简单：做 C 所做的事。字段的顺序、大小和对齐方式与你在 C 或 C++ 中期望的完全一样。
// 任何你期望通过 FFI 边界的类型都应该有 repr(C)，因为 C 是编程世界的语言框架。这对于合理地使用数据布局做更多的技巧也是必要的，比如将值重新解释为不同的类型。

// 我们强烈建议使用 rust-bindgen 和 / 或 cbindgen 来为你管理 FFI 的边界。Rust 团队与这些项目紧密合作，以确保它们能够稳健地工作，并与当前和未来关于类型布局和 reprs 的保证兼容。
// 必须记住repr(C)与 Rust 更奇特的数据布局功能的互动。由于它具有“用于 FFI”和“用于布局控制”的双重目的，repr(C)可以应用于那些如果通过 FFI 边界就会变得无意义或有问题的类型：
// 1. ZST 仍然是零大小，尽管这不是 C 语言的标准行为，而且明确违背了 C++ 中空类型的行为，即它们仍然应该消耗一个字节的空间
// 2. DST 指针（宽指针）和 tuple 在 C 语言中没有对应的概念，因此从来不是 FFI 安全的
// 3. 带有字段的枚举在 C 或 C++ 中也没有对应的概念，但是类型的有效桥接是被定义的
// 4. 如果 T 是一个 FFI 安全的非空指针类型，Option<T> 被保证具有与T相同的布局和 ABI，因此也是 FFI 安全的。截至目前，这包括&、&mut和函数指针，所有这些都不能为空。
// 5. 就 repr(C) 而言，元组结构和结构一样，因为与结构的唯一区别是字段没有命名。
// 6. repr(C) 相当于无字段枚举的 repr(u*) 之一（见下一节）。选择的大小是目标平台的 C 应用二进制接口（ABI）的默认枚举大小。请注意，C 语言中的枚举表示法是实现定义的，所以这实际上是一个“最佳猜测”。特别是，当对应的 C 代码在编译时带有某些标志时，这可能是不正确的。
// 7. 带有 repr(C) 或 repr(u*) 的无字段枚举仍然不能在没有相应变量的情况下设置为整数值，尽管这在 C 或 C++ 中是允许的行为。如果（不安全地）构造一个枚举的实例，但不与它的一个变体相匹配，这是未定义的行为(这使得详尽的匹配可以继续被编写和编译为正常行为)。

// repr(transparent)
// 这只能用于具有单个非零尺寸字段的结构（可能还有其他零尺寸字段）。其效果是，整个结构的布局和 ABI 被保证与该字段相同。
// 我们的目标是使单一字段和结构之间的转换成为可能。一个例子是 UnsafeCell，它可以被转换为它所包装的类型。（ UnsafeCell也用了一个不稳定的特性 no_niche，所以当它嵌套其它类型的时候，它的 ABI 也并没有一个稳定的保证。）
// 另外，通过 FFI 传递结构，其中内部字段类型在另一端被期望，这保证了结构的工作。特别是，这对于struct Foo(f32)总是具有与f32相同的 ABI 是必要的。
// 只有在唯一的字段为 pub 或其内存布局在文档中所承诺的情况下，该 repr 才被视为一个类型的公共 ABI 的一部分。否则，该内存布局不应被其他 crate 所依赖。

enum MyOption<T> {
    Some(T),
    None,
}

#[repr(u8)]
enum MyReprOption<T> {
    Some(T),
    None,
}

pub fn repr_c() {
    struct A {
        _a: u8,
        _b: i32,
        _c: u8,
    }

    #[repr(C)]
    struct B {
        _a: u8,
        _b: i32,
        _c: u8,
    }

    println!(
        "i8 size = {}, i16 size = {}, A size = {}",
        mem::size_of::<i8>(),
        mem::size_of::<i16>(),
        mem::size_of::<A>(),
    );

    println!("B size = {}", mem::size_of::<B>());

    struct C;
    struct D;

    // ZST 的类型大小仍然是 0
    #[repr(C)]
    struct E {
        _c: C,
        _d: D,
    }
    println!("E size = {}", mem::size_of::<E>());

    enum MyType {
        First,
        Second,
    }

    // repr(C) 和 repr(u*) 中无成员的枚举不能被赋值为一个没有对应变量的整数, 但是 C\C++ 中可以
    // let b: i32 = MyType::First;
}
