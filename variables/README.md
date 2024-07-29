# Rust Variable Declaration

## Variable

创建一个变量，在Rust声明一个变量默认是不可以被修改的。
```rust
let x = 5;
```

## Mutable variable
创建一个可修改的变量，可以在变量名前添加 `mut` 来使其可变。
```rust
let mut x = 5;
```

## Constant

声明常量使用 `const` 关键字而不是 `let`，并且 必须 注明值的类型,Rust 对常量的命名约定是在单词之间使用全大写加下划线。
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Shadowing

我们可以定义一个与之前变量同名的新变量, 第一个变量被第二个 隐藏（Shadowing） 了, 第二个变量“遮蔽”了第一个变量, 任何使用该变量名的行为都会视为是在使用第二个变量，直到第二个变量自己也被隐藏或第二个变量的作用域结束。可以用相同变量名称来隐藏一个变量，以及重复使用 `let` 关键字来多次隐藏：

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

隐藏(`Shadowing`)与将变量标记为 `mut` 是有区别的, 使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，

```rust
let spaces = "   ";
let spaces = spaces.len();
```
spaces 之前存储的是字符类型，隐蔽之后是一个新的变量存储的是数值类型，复用原来的变量名；

```rust
let mut spaces = "   ";
spaces = spaces.len();
```
这段代码将编译报错，我们不能改变变量的类型：
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```
