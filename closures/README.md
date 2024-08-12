# 闭包和迭代器

## 闭包

Rust 的 **闭包**（closures）是可以保存在一个变量中或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获被定义时所在作用域中的值。我们将展示闭包的这些功能如何复用代码和自定义行为。

### 捕获其环境

我们首先了解如何通过闭包捕获定义它的环境中的值以便之后使用。考虑如下场景：有时 T 恤公司会赠送限量版 T 恤给邮件列表中的成员作为促销。邮件列表中的成员可以选择将他们的喜爱的颜色添加到个人信息中。如果被选中的成员设置了喜爱的颜色，他们将获得那个颜色的 T 恤。如果他没有设置喜爱的颜色，他们会获赠公司现存最多的颜色的款式。

有很多种方式来实现这些。例如，使用有 `Red` 和 `Blue` 两个成员的 `ShirtColor` 枚举（出于简单考虑限定为两种颜色）。我们使用 Inventory 结构体来代表公司的库存，它有一个类型为 `Vec<ShirtColor>` 的 shirts 字段表示库存中的衬衫的颜色。Inventory 上定义的 `giveaway` 方法获取免费衬衫得主所喜爱的颜色（如有），并返回其获得的衬衫的颜色。

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

`main` 函数中定义的 `store` 还剩有两件蓝衬衫和一件红衬衫可在限量版促销活动中赠送。我们用一个期望获得红衬衫和一个没有期望的用户来调用 giveaway 方法。

再次强调，这段代码可以有多种实现方式。这里为了专注于闭包，我们会继续使用已经学习过的概念，除了 giveaway 方法体中使用了闭包。giveaway 方法获取了 `Option<ShirtColor>` 类型作为用户的期望颜色并在 `user_preference` 上调用 `unwrap_or_else` 方法。 `Option<T>` 上的方法 unwrap_or_else 由标准库定义，它获取一个没有参数、返回值类型为 T （与 `Option<T>` 的 `Some` 成员所存储的值的类型一样，这里是 ShirtColor）的闭包作为参数。如果 `Option<T>` 是 Some 成员，则 unwrap_or_else 返回 Some 中的值。`如果 Option<T>` 是 None 成员，则 unwrap_or_else 调用闭包并返回闭包的返回值。

我们将被闭包表达式 `|| self.most_stocked()` 用作 unwrap_or_else 的参数。这是一个本身不获取参数的闭包（如果闭包有参数，它们会出现在两道竖杠之间）。闭包体调用了 `self.most_stocked()`。我们在这里定义了闭包，而 unwrap_or_else 的实现会在之后需要其结果的时候执行闭包。

运行代码会打印出：

```
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

这里一个有趣的地方是我们传递了一个会在当前 Inventory 实例上调用 self.most_stocked() 的闭包。标准库并不需要知道我们定义的 Inventory 或 ShirtColor 类型或是在这个场景下我们想要用的逻辑。闭包捕获了一个 Inventory 实例的不可变引用到 self，并连同其它代码传递给 unwrap_or_else 方法。相比之下，函数就不能以这种方式捕获其环境。

### 闭包类型推断

函数与闭包还有更多区别。闭包并不总是要求像 `fn` 函数那样在参数和返回值上注明类型。函数中需要类型注解是因为它们是暴露给用户的显式接口的一部分。严格定义这些接口对保证所有人都对函数使用和返回值的类型理解一致是很重要的。与此相比，闭包并不用于这样暴露在外的接口：它们储存在变量中并被使用，不用命名它们或暴露给库的用户调用。

闭包通常很短，并只关联于小范围的上下文而非任意情境。在这些有限制的上下文中，编译器能可靠地推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样（同时也有编译器需要闭包类型注解的罕见情况）。

类似于变量，如果我们希望增加明确性和清晰度也可以添加类型标注，坏处是使代码变得更啰嗦（相对于严格必要的代码），闭包标注类型看起来如函数的定义一样。

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

有了类型注解闭包的语法就更类似函数了。如下是一个对其参数加一的函数的定义与拥有相同行为闭包语法的纵向对比。这里增加了一些空格来对齐相应部分。这展示了除了使用竖线以及一些可选语法外，闭包语法与函数语法有多么地相似：

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行展示了一个函数定义，第二行展示了一个完整标注的闭包定义。第三行闭包定义中省略了类型注解，而第四行去掉了可选的大括号，因为闭包体只有一个表达式。这些都是有效的闭包定义，并在调用时产生相同的行为。调用闭包是 `add_one_v3` 和 `add_one_v4` 能够编译的必要条件，因为类型将从其用法中推断出来。这类似于 let v = Vec::new();，Rust 需要类型注解或是某种类型的值被插入到 Vec 才能推断其类型。

编译器会为闭包定义中的每个参数和返回值推断一个具体类型。例如仅仅将参数作为返回值的简短的闭包定义。除了作为示例的目的这个闭包并不是很实用。注意这个定义没有增加任何类型注解，所以我们可以用任意类型来调用这个闭包。但是如果尝试调用闭包两次，第一次使用 String 类型作为参数而第二次使用 u32，则会得到一个错误：

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

编译器给出如下错误：

```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected `String`, found integer
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` (bin "closure-example") due to 1 previous error
```

第一次使用 `String` 值调用 example_closure 时，编译器推断这个闭包中 `x` 的类型以及返回值的类型是 String。接着这些类型被锁定进闭包 example_closure 中，如果尝试对同一闭包使用不同类型则就会得到类型错误。


### 捕获值所有权

闭包可以通过三种方式捕获其环境，它们直接对应到函数获取参数的三种方式：不可变借用，可变借用和获取所有权。闭包会根据函数体中如何使用被捕获的值决定用哪种方式捕获。

定义一个捕获名为 list 的 vector 的不可变引用的闭包，因为只需不可变引用就能打印其值：

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

这个示例也展示了变量可以绑定一个闭包定义，并且之后可以使用变量名和括号来调用闭包，就像变量名是函数名一样。

因为同时可以有多个 `list` 的不可变引用，所以在闭包定义之前，闭包定义之后调用之前，闭包调用之后代码仍然可以访问 list。代码可以编译、运行并打印：

```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

我们修改闭包体让它向 list vector 增加一个元素。闭包现在捕获一个可变引用：

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```

代码可以编译、运行并打印：

```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
```

注意在 `borrows_mutably` 闭包的定义和调用之间不再有 `println!`，当 borrows_mutably 定义时，它捕获了 list 的可变引用。闭包在被调用后就不再被使用，这时可变借用结束。因为当可变借用存在时不允许有其它的借用，所以在闭包定义和调用之间不能有不可变引用来进行打印。可以尝试在这里添加 println! 看看你会得到什么报错信息！

即使闭包体不严格需要所有权，如果希望强制闭包获取它用到的环境中值的所有权，可以在参数列表前使用 move 关键字。

在将闭包传递到一个新的线程时这个技巧很有用，它可以移动数据所有权给新线程。用 move 关键字的闭包来生成新的线程。在一个新的线程而非主线程中打印 vector：

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

我们生成了新的线程，给这个线程一个闭包作为参数运行，闭包体打印出列表。闭包通过不可变引用捕获 list，因为这是打印列表所需的最少的访问。这个例子中，尽管闭包体依然只需要不可变引用，我们还是在闭包定义前写上 move 关键字来指明 list 应当被移动到闭包中。新线程可能在主线程剩余部分执行完前执行完，或者也可能主线程先执行完。如果主线程维护了 list 的所有权但却在新线程之前结束并且丢弃了 list，则在线程中的不可变引用将失效。因此，编译器要求 list 被移动到在新线程中运行的闭包中，这样引用就是有效的。试着去掉 move 关键字或在闭包被定义后在主线程中使用 list 看看你会得到什么编译器报错！

### 三种闭包 trait

一旦一个闭包从定义闭包的环境中捕获了一个引用或捕获了一个值的所有权（从而影响到将内容 _移入_ 闭包，如有）， 在定义闭包的代码主体中，对闭包捕获的值在后续闭包体计算时发生什么变化（从而影响将内容被 _移出_ 闭包，如有）。闭包体可以做以下事情： 捕获值移入闭包，修改捕获的值，既不移动也不修改值，将捕获的值移出闭包，或者不从环境中捕获值。

1. `FnOnce` 适用于只能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。将捕获的值移入闭包体的闭包只能实现 `FnOnce` trait，不能实现 `Fn` trait , 因为它只能被调用一次。
2. `FnMut` 适用于不将捕获的值移入且没有移出的闭包体，但是它可以修改捕获的值。这类闭包可以被调用多次。
3. `Fn` 适用于既不将被捕获的值移入闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。

让我们来看看使用的在 `Option<T>` 上的 unwrap_or_else 方法的定义：

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

`T` 是表示 `Option` 中 `Some` 成员中的值的类型的泛型。类型 `T` 也是 `unwrap_or_else` 函数的返回值类型：举例来说，在 `Option<String>` 上调用 unwrap_or_else 会得到一个 `String`。

接着注意到 unwrap_or_else 函数有额外的泛型参数 `F`。 F 是 f 参数（即调用 unwrap_or_else 时提供的闭包）的类型。

泛型 F 的 trait bound 是 `FnOnce() -> T`，这意味着 F 必须能够被调用一次，没有参数并返回一个 T。在 trait bound 中使用 FnOnce 表示 unwrap_or_else 将最多调用 f 一次。在 unwrap_or_else 的函数体中可以看到，如果 Option 是 Some，f 不会被调用。如果 Option 是 None，f 将会被调用一次。由于所有的闭包都实现了 FnOnce，unwrap_or_else 能接收绝大多数不同类型的闭包，十分灵活。

> 注意：函数也可以实现所有的三种 `Fn` traits。如果我们要做的事情不需要从环境中捕获值，则可以在需要某种实现了 Fn trait 的东西时使用函数而不是闭包。举个例子，可以在 Option<Vec<T>> 的值上调用 `unwrap_or_else(Vec::new)` 以便在值为 None 时获取一个新的空的 vector。

现在让我们来看定义在 slice 上的标准库方法 `sort_by_key`，看看它与 unwrap_or_else 的区别以及为什么 sort_by_key 使用 `FnMut` 而不是 `FnOnce` trait bound。这个闭包以一个 slice 中当前被考虑的元素的引用作为参数，返回一个可以用来排序的 K 类型的值。当你想按照 slice 中元素的某个属性来进行排序时这个函数很有用。在一个 Rectangle 实例的列表，我们使用 sort_by_key 按 Rectangle 的 width 属性对它们从低到高排序：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
```

代码输出：

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

`sort_by_key` 被定义为接收一个 `FnMut` 闭包的原因是它会多次调用这个闭包：每个 slice 中的元素调用一次。闭包 `|r| r.width` 不捕获、修改或将任何东西移出它的环境，所以它满足 trait bound 的要求。

作为对比，以下示例展示了一个只实现了 FnOnce trait 的闭包（因为它从环境中移出了一个值）的例子。编译器不允许我们在 sort_by_key 上使用这个闭包：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}
```

这是一个刻意构造的、繁琐的方式，它尝试统计排序 list 时 sort_by_key 被调用的次数（并不能工作）。该代码尝试在闭包的环境中向 `sort_operations` vector 放入 `value` — 一个 `String` 来实现计数。闭包捕获了 value 然后通过转移 value 的所有权的方式将其移出闭包给到 sort_operations vector。这个闭包可以被调用一次，尝试再次调用它将报错。因为这时 value 已经不在闭包的环境中，无法被再次放到 sort_operations 中！因而，这个闭包只实现了 `FnOnce`。由于要求闭包必须实现 `FnMut`，因此尝试编译这个代码将得到报错：`value` 不能被移出闭包：

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:18:30
   |
15 |     let value = String::from("closure called");
   |         ----- captured outer variable
16 |
17 |     list.sort_by_key(|r| {
   |                      --- captured by this `FnMut` closure
18 |         sort_operations.push(value);
   |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` (bin "rectangles") due to 1 previous error
```

报错指向了闭包体中将 value 移出环境的那一行。要修复这里，我们需要改变闭包体让它不将值移出环境。在环境中保持一个计数器并在闭包体中增加它的值是计算 sort_by_key 被调用次数的一个更简单直接的方法。闭包可以在 sort_by_key 中使用，因为它只捕获了 num_sort_operations 计数器的可变引用，这就可以被调用多次。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
```

当定义或使用用到闭包的函数或类型时，Fn trait 十分重要。在下个小节中，我们将会讨论迭代器。许多迭代器方法都接收闭包参数，因而在继续前先记住这些闭包的细节！

## 迭代器

迭代器模式允许你对一个序列的项进行某些处理。**迭代器**（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。

在 Rust 中，迭代器是 **惰性的**（lazy），这意味着在调用消费迭代器的方法之前不会执行任何操作。例如，通过调用定义于 `Vec` 上的 `iter` 方法在一个 vector v1 上创建了一个迭代器。这段代码本身没有任何用处：

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

迭代器被储存在 `v1_iter` 变量中。一旦创建迭代器之后，可以选择用多种方式利用它。我们使用 for 循环来遍历一个数组并在每一个项上执行了一些代码。在底层它隐式地创建并接着消费了一个迭代器，不过直到现在我们都一笔带过了它具体是如何工作的。

将迭代器的创建和 for 循环中的使用分开。迭代器被储存在 v1_iter 变量中，而这时没有进行迭代。一旦 for 循环开始使用 v1_iter，接着迭代器中的每一个元素被用于循环的一次迭代，这会打印出其每一个值：

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}
```

在标准库中没有提供迭代器的语言中，我们可能会使用一个从 0 开始的索引变量，使用这个变量索引 vector 中的值，并循环增加其值直到达到 vector 的元素数量。

迭代器为我们处理了所有这些逻辑，这减少了重复代码并消除了潜在的混乱。另外，迭代器的实现方式提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构。让我们看看迭代器是如何做到这些的。

### Iterator trait

迭代器都实现了一个叫做 `Iterator` 的定义于标准库的 trait。这个 trait 的定义看起来像这样：

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
```

注意这里有一个我们还未讲到的新语法：`type Item` 和 `Self::Item`，它们定义了 trait 的 **关联类型**（associated type）。现在只需知道这段代码表明实现 Iterator trait 要求同时定义一个 `Item` 类型，这个 Item 类型被用作 `next` 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。

`next` 是 `Iterator` 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 `Some` 中，当迭代器结束时，它返回 `None`。

可以直接调用迭代器的 next 方法；有一个测试展示了重复调用由 vector 创建的迭代器的 next 方法所得到的值：

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

注意 `v1_iter` 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 **消费**（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。

另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。`iter` 方法生成一个不可变引用的迭代器。如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 `into_iter` 而不是 iter。类似的，如果我们希望迭代可变引用，则可以调用 `iter_mut` 而不是 iter。

### 消费迭代器

Iterator trait 有一系列不同的由标准库提供默认实现的方法；你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 `next` 方法的原因。

这些调用 next 方法的方法被称为 **消费适配器**（consuming adaptors），因为调用它们会消耗迭代器。一个消费适配器的例子是 `sum` 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。展示一个 sum 方法使用的测试：

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。

### 迭代器方法

Iterator trait 中定义了另一类方法，被称为 **迭代器适配器**（iterator adaptors），它们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

展示一个调用迭代器适配器方法 `map` 的例子，该 map 方法使用闭包来调用每个元素以生成新的迭代器。这里的闭包创建了一个新的迭代器，对其中 vector 中的每个元素都被加 1。：

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

不过这些代码会产生一个警告:

```
$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = v1.iter().map(|x| x + 1);
  |     +++++++

warning: `iterators` (bin "iterators") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`
```

代码实际上并没有做任何事；所指定的闭包从未被调用过。警告提醒了我们为什么：迭代器适配器是惰性的，而这里我们需要消费迭代器。

为了修复这个警告并消费迭代器获取有用的结果，我们将使用结合 env::args 使用的 `collect` 方法。这个方法消费迭代器并将结果收集到一个数据结构中。

我们将遍历由 map 调用生成的迭代器的结果收集到一个 vector 中，它将会含有原始 vector 中每个元素加 1 的结果：

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

因为 `map` 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。这是一个展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的绝佳例子。

可以链式调用多个迭代器适配器来以一种可读的方式进行复杂的操作。不过因为所有的迭代器都是惰性的，你需要调用一个消费适配器方法从迭代器适配器调用中获取结果。

### 使用捕获值的闭包

很多迭代器适配器接受闭包作为参数，而通常指定为迭代器适配器参数的闭包会是捕获其环境的闭包。

作为一个例子，我们使用 `filter` 方法来获取一个闭包。该闭包从迭代器中获取一项并返回一个 bool。如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会被包含。

使用 filter 和一个捕获环境中变量 `shoe_size` 的闭包来遍历一个 `Shoe` 结构体集合。它只会返回指定大小的鞋子。

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

`shoes_in_size` 函数获取一个鞋子 vector 的所有权和一个鞋子大小作为参数。它返回一个只包含指定大小鞋子的 vector。

shoes_in_size 函数体中调用了 `into_iter` 来创建一个获取 vector 所有权的迭代器。接着调用 filter 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器。

闭包从环境中捕获了 shoe_size 变量并使用其值与每一只鞋的大小作比较，只保留指定大小的鞋子。最终，调用 collect 将迭代器适配器返回的值收集进一个 vector 并返回。

闭包从环境中捕获了 shoe_size 变量并使用其值与每一只鞋的大小作比较，只保留指定大小的鞋子。最终，调用 collect 将迭代器适配器返回的值收集进一个 vector 并返回。

这个测试展示当调用 `shoes_in_size` 时，我们只会得到与指定值相同大小的鞋子。

## 应用优化

有了这些关于迭代器的新知识，我们可以使用迭代器来改进 `minigrep` 项目的实现来使得代码更简洁明了。让我们看看迭代器如何能够改进 `Config::build` 函数和 `search` 函数的实现。

### 使用迭代器并去掉 clone

minigrep 我们增加了一些代码获取一个 String slice 并创建一个 Config 结构体的实例，它们索引 slice 中的值并克隆这些值以便 Config 结构体可以拥有这些值。

```rust
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

那时我们说过不必担心低效的 `clone` 调用了，因为将来可以对它们进行改进。好吧，就是现在！

起初这里需要 clone 的原因是参数 args 中有一个 String 元素的 slice，而 build 函数并不拥有 args。为了能够返回 Config 实例的所有权，我们需要克隆 Config 中字段 `query` 和 `file_path` 的值，这样 Config 实例就能拥有这些值。

在学习了迭代器之后，我们可以将 build 函数改为获取一个有所有权的迭代器作为参数而不是借用 slice。我们将使用迭代器功能之前检查 slice 长度和索引特定位置的代码。这会明确 Config::build 的工作因为迭代器会负责访问这些值。

一旦 `Config::build` 获取了迭代器的所有权并不再使用借用的索引操作，就可以将迭代器中的 String 值移动到 Config 中，而不是调用 clone 分配新的空间。

```rust
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}
```

`env::args` 函数返回一个迭代器！不同于将迭代器的值收集到一个 vector 中接着传递一个 slice 给 Config::build，现在我们直接将 env::args 返回的迭代器的所有权传递给 Config::build。

接下来需要更新 Config::build 的定义，将 Config::build 的签名改为如下：

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        // --snip--

}
```

env::args 函数的标准库文档显示，它返回的迭代器的类型为 `std::env::Args`，同时这个类型实现了 `Iterator` trait 并返回 `String` 值。

我们已经更新了 Config::build 函数的签名，因此参数 args 有一个带有 trait bound `impl Iterator<Item = String>` 的泛型类型，而不是 `&[String]`。这里用到了 “trait 作为参数” 部分讨论过的 impl Trait 语法意味着 args 可以是任何实现了 Iterator 的类型并返回 String 项（item）。

因为我们拥有 args 的所有权，并且将通过对其进行迭代来改变 args ，所以我们可以将 mut 关键字添加到 args 参数的规范中以使其可变。

接下来，我们将修改 Config::build 的内容。因为 args 实现了 Iterator trait，因此我们知道可以对其调用 next 方法！

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

请记住 env::args 返回值的第一个值是程序的名称。我们希望忽略它并获取下一个值，所以首先调用 next 并不对返回值做任何操作。之后对希望放入 Config 中字段 query 调用 next。如果 next 返回 Some，使用 match 来提取其值。如果它返回 None，则意味着没有提供足够的参数并通过 Err 值提早返回。对 file_path 值进行同样的操作。

### 使用迭代适配器

minigrep 项目中其他可以利用迭代器的地方是 search 函数， 此函数的定义：

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

可以通过使用 _迭代器适配器_ 方法来编写更简明的代码。这也避免了一个可变的中间 results vector 的使用。函数式编程风格倾向于最小化可变状态的数量来使代码更简洁。去掉可变状态可能会使得将来进行并行搜索的增强变得更容易，因为我们不必管理 results vector 的并发访问。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

回忆 search 函数的目的是返回所有 contents 中包含 query 的行。可以使用 filter 适配器只保留 `line.contains(query)` 返回 true 的那些行。接着使用 collect 将匹配行收集到另一个 vector 中。这样就容易多了！尝试对 search_case_insensitive 函数做出同样的使用迭代器方法的修改吧。

## forloop vs iterator

接下来的逻辑问题就是在代码中应该选择哪种风格：是使用 for loop 的原始实现还是使用示例 iterator 中使用迭代器的版本？大部分 Rust 程序员倾向于使用迭代器风格。开始这有点难以理解，不过一旦你对不同迭代器的工作方式有了感觉之后，迭代器可能会更容易理解。相比摆弄不同的循环并创建新 vector，（迭代器）代码则更关注循环的目的。这抽象掉那些老生常谈的代码，这样就更容易看清代码所特有的概念，比如迭代器中每个元素必须面对的过滤条件。

### 性能比较

为了决定使用哪个实现，我们需要知道哪个版本的 search 函数更快一些：是直接使用 for 循环的版本还是使用迭代器的版本。

我们运行了一个性能测试，通过将阿瑟·柯南·道尔的“福尔摩斯探案集”的全部内容加载进 String 并寻找其中的单词 “the”。如下是 for 循环版本和迭代器版本的 search 函数的性能测试结果：

```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

结果迭代器版本还要稍微快一点！这里我们将不会查看性能测试的代码，我们的目的并不是为了证明它们是完全等同的，而是得出一个怎样比较这两种实现方式性能的基本思路。

对于一个更全面的性能测试，将会检查不同长度的文本、不同的搜索单词、不同长度的单词和所有其他的可变情况。这里所要表达的是：迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能的代码。迭代器是 Rust 的 **零成本抽象**（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销，它与本贾尼·斯特劳斯特卢普（C++ 的设计和实现者）在 “Foundations of C++”（2012）中所定义的 **零开销**（zero-overhead）如出一辙：

> In general, C++ implementations obey the zero-overhead principle: What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better.
>
> - Bjarne Stroustrup "Foundations of C++"
>
> 从整体来说，C++ 的实现遵循了零开销原则：你不需要的，无需为它们买单。更有甚者的是：你需要的时候，也不可能找到其他更好的代码了。
>
> - 本贾尼·斯特劳斯特卢普 "Foundations of C++"

作为另一个例子，这里有一些取自于音频解码器的代码。解码算法使用线性预测数学运算（linear prediction mathematical operation）来根据之前样本的线性函数预测将来的值。这些代码使用迭代器链对作用域中的三个变量进行某种数学计算：一个叫 buffer 的数据 slice、一个有 12 个元素的数组 coefficients、和一个代表位移位数的 qlp_shift。例子中声明了这些变量但并没有提供任何值；虽然这些代码在其上下文之外没有什么意义，不过仍是一个简明的现实中的例子，来展示 Rust 如何将高级概念转换为底层代码：

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

为了计算 `prediction` 的值，这些代码遍历了 `coefficients` 中的 12 个值，使用 `zip` 方法将系数与 `buffer` 的前 12 个值组合在一起。接着将每一对值相乘，再将所有结果相加，然后将总和右移 qlp_shift 位。

像音频解码器这样的程序通常最看重计算的性能。这里，我们创建了一个迭代器，使用了两个适配器，接着消费了其值。那么这段 Rust 代码将会被编译为什么样的汇编代码呢？好吧，在编写本书的这个时候，它被编译成与手写的相同的汇编代码。遍历 coefficients 的值完全用不到循环：Rust 知道这里会迭代 12 次，所以它“展开”（unroll）了循环。展开是一种将循环迭代转换为重复代码，并移除循环控制代码开销的代码优化技术。

所有的系数都被储存在了寄存器中，这意味着访问它们非常快。这里也没有运行时数组访问边界检查。所有这些 Rust 能够提供的优化使得结果代码极为高效。现在知道这些了，请放心大胆的使用迭代器和闭包吧！它们使得代码看起来更高级，但并不为此引入运行时性能损失。




