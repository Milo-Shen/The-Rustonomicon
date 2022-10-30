// Exotically Sized Types
// https://doc.rust-lang.org/nomicon/exotic-sizes.html
// https://nomicon.purewhite.io/exotic-sizes.html

// DST 动态尺寸类型
// 1. DST 大小是未知的，因此只能通过指针来访问他
// 2. 一个指向 DST 的指针是一个 “胖” 指针，它包含指针本身和一些额外的信息
// 3. Rust 主要提供 2 种 DST 类型: trait 对象和 slice
// 4. Trait 对象中具体的类型被擦除了, 取而代之的是运行期的一个虚函数表, 因此 trait 对象的额外信息只有: 一个指向虚函数表的指针
// 5. slice 对应的额外信息就是指向的元素的数量

// DST 不是一个普通的类型，因为它们没有编译时静态可知的大小，它们只能存在于一个指针之后。任何指向 DST 的指针都会变成一个包含了完善 DST 类型信息的胖指针（详情见下方）。
// Rust 暴露了两种主要的 DST 类型：
// 1. trait objects: dyn MyTrait
// 2. slices：[T]、str及其他

// Trait 对象代表某种类型，实现了它所指定的 Trait。确切的原始类型被删除，以利于运行时的反射，其中包含使用该类型的所有必要信息的 vtable。
// 补全 Trait 对象指针所需的信息是 vtable 指针，被指向的对象的运行时的大小可以从 vtable 中动态地获取。