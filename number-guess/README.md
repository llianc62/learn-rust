# number guess

创建一个可修改变量
```rust
let mut guess = String::new()
```

为了获取用户输入并打印结果作为输出，我们需要将 io输入/输出库引入当前作用域。io 库来自于标准库，也被称为 std：
```rust
use std::io
```

接收用户输入
回忆一下，我们在程序的第一行使用 use std::io; 从标准库中引入了输入/输出功能。现在调用 io 库中的函数 stdin：

```rust
io::stdin()
    .read_line(&mut guess)
```

## rand
Cargo 对外部 crate 的运用是其真正的亮点所在。在我们使用 rand 编写代码之前，需要修改 Cargo.toml 文件，引入一个 rand 依赖。现在打开这个文件并将下面这一行添加到 [dependencies] 片段标题之下。在当前版本下，请确保按照我们这里的方式指定 rand，否则本教程中的示例代码可能无法工作。

文件名：`Cargo.toml`
```toml
[dependencies]
rand = "0.8.5"
```

首先，我们新增了一行 use rand::Rng;。Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。

```rust
use rand::Rng;
```

接下来，我们在中间还新增加了两行。第一行调用了 `rand::thread_rng` 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。接着调用随机数生成器的 `gen_range` 方法。这个方法由 use rand::Rng 语句引入到作用域的 Rng trait 定义。gen_range 方法获取一个范围表达式（range expression）作为参数，并生成一个在此范围之间的随机数。这里使用的这类范围表达式使用了 start..=end 这样的形式，也就是说包含了上下端点，所以需要指定 1..=100 来请求一个 1 和 100 之间的数。
```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

> 运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。例如，假设你对 rand crate 中的其他功能感兴趣，你可以运行 cargo doc --open 并点击左侧导航栏中的 rand。

## match

首先我们增加了另一个 use 声明，从标准库引入了一个叫做 std::cmp::Ordering 的类型到作用域中。 Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果。

```rust
use std::cmp::Ordering;
```

使用 `Ordering` 类型，`cmp` 方法用来比较两个值并可以在任何可比较的值上调用。它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较。然后它会返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员。使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

