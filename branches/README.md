# Control Flow

## if 表达式

if 表达式允许根据条件执行不同的代码分支。你提供一个条件并表示 “如果条件满足，运行这段代码；如果条件不满足，不运行这段代码。

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

所有的 if 表达式都以 if 关键字开头，其后跟一个条件。在条件为 true 时希望执行的代码块位于紧跟条件之后的大括号中。if 表达式中与条件关联的代码块有时被叫做 arms。也可以包含一个可选的 else 表达式来提供一个在条件为 false 时应当执行的代码块，如果不提供 else 表达式并且条件为 false 时，程序会直接忽略 if 代码块并继续执行下面的代码。

**使用 else if 处理多重条件**

可以将 else if 表达式与 if 和 else 组合来实现多重条件。

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

在 let 语句中使用 if
因为 if 是一个表达式，我们可以在 let 语句的右侧使用它，

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## loop

loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

从循环返回值
loop 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。然而你可能会需要将操作的结果传递给其它的代码。如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

**循环标签**：在多个循环之间消除歧义
如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。你可以选择在一个循环上指定一个 循环标签（`loop label`），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。下面是一个包含两个嵌套循环的示例

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

## while condition

while：程序循环三次，每次数字都减一。接着，在循环结束后，打印出另一个信息并退出。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

## for range

使用 for 循环来对一个集合的每个元素执行一些代码。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```



