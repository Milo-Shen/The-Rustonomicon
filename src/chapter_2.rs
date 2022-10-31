use std::mem;

// Exotically Sized Types
// https://doc.rust-lang.org/nomicon/exotic-sizes.html
// https://nomicon.purewhite.io/exotic-sizes.html

// DST 动态尺寸类型
// 1. DST 大小是未知的，因此只能通过指针来访问他
// 2. 一个指向 DST 的指针是一个 “胖” 指针，它包含指针本身和一些额外的信息
// 3. Rust 主要提供 2 种 DST 类型: trait 对象和 slice
// 4. Trait 对象中具体的类型被擦除了, 取而代之的是运行期的一个虚函数表, 因此 trait 对象的额外信息只有: 一个指向虚函数表的指针
// 5. slice 对应的额外信息就是指向的元素的数量

// 引申: trait 对象的胖指针大小是多大 ?
// 应该是 2 个指针的大小, 一个是指向具体的数据结构, 另一个指针指向虚函数表

// 引申: slice 的胖指针有多大 ?
// 也是 2 个指针大小, 一个指针的大小, 加上一个字段记录元素的个数

// DST 不是一个普通的类型，因为它们没有编译时静态可知的大小，它们只能存在于一个指针之后。任何指向 DST 的指针都会变成一个包含了完善 DST 类型信息的胖指针（详情见下方）。
// Rust 暴露了两种主要的 DST 类型：
// 1. trait objects: dyn MyTrait
// 2. slices：[T]、str及其他

// Trait 对象代表某种类型，实现了它所指定的 Trait。确切的原始类型被删除，以利于运行时的反射，其中包含使用该类型的所有必要信息的 vtable。
// 补全 Trait 对象指针所需的信息是 vtable 指针，被指向的对象的运行时的大小可以从 vtable 中动态地获取。

// 一个 slice 只是一些只读的连续存储——通常是一个数组或Vec。补全一个 slice 指针所需的信息只是它所指向的元素的数量，指针的运行时大小只是静态已知元素的大小乘以元素的数量。
// 泛型类型是在编译的时候知道大小

struct A<'a> {
    _a: i32,
    _b: &'a [u8],
}

trait MyTrait {
    fn test();
}

pub fn dst() {
    let array: [u8; 10] = [1; 10];
    let s = &array[..];

    println!("s zise = {}", mem::size_of_val(s));
    println!("&s zise = {}", mem::size_of_val(&s));
    println!(
        "i32 size = {}, &i32 size = {}",
        mem::size_of::<i32>(),
        mem::size_of::<&i32>()
    );
    println!(
        "i64 size = {}, &i64 size = {}",
        mem::size_of::<i64>(),
        mem::size_of::<&i64>()
    );
    println!(
        "A size = {}, &A size = {}",
        mem::size_of::<A>(),
        mem::size_of::<&A>()
    );

    // 结构实际上可以直接存储一个 DST 作为其最后一个字段，但这也会使它们自身成为一个 DST:

    // 不能直接存储在栈上
    struct MySuperSlice {
        info: u32,
        data: [u8],
    }

    // 如果这样的类型没有方法来构造它，那么它在很大程度上来看是没啥用的。目前，唯一支持的创建自定义 DST 的方法是使你的类型成为泛型，并执行非固定大小转换（unsizing coercion）:
    struct MySuperSliceable<T: ?Sized> {
        info: u32,
        data: T,
    }

    let sized: MySuperSliceable<[u8; 8]> = MySuperSliceable {
        info: 17,
        data: [0; 8],
    };

    let dynamic: &MySuperSliceable<[u8]> = &sized;
    println!("{} {:?}", dynamic.info, &dynamic.data);
}

// 零大小类型 (ZSTs)
// Rust 允许一种类型的尺寸大小为 0
// 1. 有成员 = 没有尺寸, 例子: struct Foo;
// 2. 所有成员都没有尺寸 = 没有尺寸, 例子:
// struct Foo;
// struct Bzx {
//     foo: Foo,
//     qux: (),      // 空元组没有尺寸
//     baz: [u8; 0], // 空数组没有尺寸
// }
// 3. 安全代码不用关注 ZST, 但是非安全代码必须考虑尺寸类型带来的影响

struct Nothing; // 无字段意味着没有大小

// 所有字段都无大小意味着整个结构体无大小
struct LotsOfNothing {
    foo: Nothing,
    qux: (),      // 空元组无大小
    baz: [u8; 0], // 空数组无大小
}

// 就其本身而言，零尺寸类型（ZSTs）由于显而易见的原因是相当无用的。
// 然而，就像 Rust 中许多奇怪的布局选择一样，它们的潜力在通用语境中得以实现。
// 在 Rust 中，任何产生或存储 ZST 的操作都可以被简化为无操作（no-op）。
// 首先，存储它甚至没有意义——它不占用任何空间。
// 另外，这种类型的值只有一个，所以任何加载它的操作都可以直接凭空产生它——这也是一个无操作（no-op），因为它不占用任何空间。

// 这方面最极端的例子之一是 Set 和 Map。
// 给定一个Map<Key, Value>，通常可以实现一个Set<Key>，作为Map<Key, UselessJunk>的一个薄封装。
// 在许多语言中，这将需要为无用的封装分配空间，并进行存储和加载无用封装的工作，然后将其丢弃。对于编译器来说，证明这一点是不必要的，是一个困难的分析。

// 然而在 Rust 中，我们可以直接说Set<Key> = Map<Key, ()>。
// 现在 Rust 静态地知道每个加载和存储都是无用的，而且没有分配有任何大小。
// 其结果是，单例化的代码基本上是 HashSet 的自定义实现，而没有 HashMap 要支持值所带来的开销。

// 安全的代码不需要担心 ZST，但是不安全的代码必须小心没有大小的类型的后果。特别是，指针偏移是无操作的，而分配器通常需要一个非零的大小。
// 请注意，对 ZST 的引用（包括空片），就像所有其他的引用一样，必须是非空的，并且适当地对齐。解引用 ZST 的空指针或未对齐指针是未定义的行为，就像其他类型的引用一样。
pub fn zst() {
    println!(
        "Nothing size = {}, &Nothing size = {}",
        mem::size_of::<Nothing>(),
        mem::size_of::<&Nothing>()
    );

    println!(
        "LotsOfNothing size = {}, &LotsOfNothing size = {}",
        mem::size_of::<Nothing>(),
        mem::size_of::<&Nothing>()
    );
}

// 空类型
// Rust 甚至也支持不能被实例化的类型, 这种类型只有类型, 而没有对应的值
// 空类型可以通过指定没有变量的枚举来声明, 例如: enum Void {}, 没有变量即为空类型
// 创建指向空类型的裸指针实际上是合法的, 但是对他解引用是一个未定义的行为，因为这么做没有任何意义
// 也就是说, 我们可以使用 *const Void 来模拟 C 语言的 Void * 类型, 但是使用 *const() 却不会得到任何东西, 因为这个函数对于随机解引用是安全的。

// Rust 还允许声明不能被实例化的类型。这些类型只能在类型层讨论，而不能在值层讨论。空类型可以通过指定一个没有变体的枚举来声明：
enum Void {} // 没有变体的类型 = 空类型

// 空类型甚至比 ZST 更加边缘化。空类型的主要作用是为了让某个类型不可达。
// 例如，假设一个 API 需要在一般情况下返回一个结果，但一个特定的情况实际上是不可能的。实际上可以通过返回一个Result<T, Void>来在类型级别上传达这个信息。
// API 的消费者可以放心地 unwrap 这样一个结果，因为他们知道这个值在本质上不可能是Err，因为这需要提供一个Void类型的值。

// 原则上，Rust 可以基于这个事实做一些有趣的分析和优化，例如，Result<T, Void> 只表示为 T，因为 Err 的情况实际上并不存在（严格来说，这只是一种优化，并不保证，所以例如将一个转化为另一个仍然是 UB）。
pub fn et() {
    enum Void {}

    let res: Result<u32, Void> = Ok(0);

    // 不存在 Err 的情况，所以 Ok 实际上永远都能匹配成功
    // 现在的新版本 rust 不允许这种写法了
    // let Ok(num) = res;
}

// 我们建议不要用 *const Void 来模拟 C 的 void* 类型。很多人之前这样做，但很快就遇到了麻烦，因为 Rust 没有任何安全防护措施来防止用不安全的代码来实例化空类型，如果你这样做了，就是未定义行为。
// 因为开发者有将原始指针转换为引用的习惯，而构造一个 &Void 也是未定义行为，所以这尤其成问题。
// *const()（或等价物）对void*来说效果相当好，可以做成引用而没有任何安全问题。它仍然不能阻止你试图读取或写入数值，但至少它可以编译成一个 no-op 而不是 UB。

// 外部类型
// 有一个已被接受的 RFC 来增加具有未知大小的适当类型，称为 extern 类型，这将让 Rust 开发人员更准确地模拟像 C 的void*和其他“声明但从未定义”的类型。然而，截至 Rust 2018，该功能在size_of_val::<MyExternType>()应该如何表现方面遇到了一些问题。
