# 构建命令行程序

本章既是一个目前所学的很多技能的概括，也是一个更多标准库功能的探索。我们将构建一个与文件和命令行输入/输出交互的命令行工具来练习现在一些你已经掌握的 Rust 技能。

Rust 的运行速度、安全性、单二进制文件输出和跨平台支持使其成为创建命令行程序的绝佳选择，所以我们的项目将创建一个我们自己版本的经典命令行搜索工具：`grep`。grep 是 “**G**lobally search a **R**egular **E**xpression and **P**rint.” 的首字母缩写。grep 最简单的使用场景是在特定文件中搜索指定字符串。为此，grep 获取一个文件路径和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

在这个过程中，我们会展示如何让我们的命令行工具利用很多命令行工具中用到的终端功能。读取环境变量来使得用户可以配置工具的行为。打印到标准错误控制流（stderr）而不是标准输出（stdout），例如这样用户可以选择将成功输出重定向到文件中的同时仍然在屏幕上显示错误信息。

一位 Rust 社区的成员，Andrew Gallant，已经创建了一个功能完整且非常快速的 grep 版本，叫做 ripgrep。相比之下，我们的版本将非常简单，本章将教会你一些帮助理解像 ripgrep 这样真实项目的背景知识。

我们的 grep 项目将会结合之前所学的一些内容：

- 代码组织
- vector 和字符串
- 错误处理
- 合理的使用 trait 和生命周期
- 测试

另外还会简要的讲到闭包、迭代器和 trait 对象。

## 接收命令行参数

一如既往使用 cargo new 新建一个项目，我们称之为 minigrep 以便与可能已经安装在系统上的 grep 工具相区别：

```bash
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

第一个任务是让 `minigrep` 能够接受两个命令行参数：文件路径和要搜索的字符串。也就是说我们希望能够使用 cargo run、要搜索的字符串和被搜索的文件的路径来运行程序，像这样：

```bash
$ cargo run -- searchstring example-filename.txt
```

### 读取参数值

现在 cargo new 生成的程序忽略任何传递给它的参数。Crates.io 上有一些现成的库可以帮助我们接受命令行参数，不过我们正在学习这些内容，让我们自己来实现一个。

为了确保 minigrep 能够获取传递给它的命令行参数的值，我们需要一个 Rust 标准库提供的函数 `std::env::args`。这个函数返回一个传递给程序的命令行参数的 **迭代器**（iterator）。只需理解迭代器的两个细节：迭代器生成一系列的值，可以在迭代器上调用 collect 方法将其转换为一个集合，比如包含所有迭代器产生元素的 vector。

读取任何传递给它的命令行参数并将其收集到一个 vector 中:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

首先使用 `use` 语句来将 `std::env` 模块引入作用域以便可以使用它的 `args` 函数。注意 `std::env::args` 函数被嵌套进了两层模块中。当所需函数嵌套了多于一层模块时，通常将父模块引入作用域，而不是其自身。这便于我们利用 std::env 中的其他函数。这比增加了 use std::env::args; 后仅仅使用 args 调用函数要更明确一些，因为 args 容易被错认成一个定义于当前模块的函数。

> **args 函数和无效的 Unicode**
>
> 注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。这个函数返回 OsString 值而不是 String 值。这里出于简单考虑使用了 std::env::args，因为 OsString 值每个平台都不一样而且比 String 值处理起来更为复杂。

在 `main` 函数的第一行，我们调用了 `env::args`，并立即使用 `collect` 来创建了一个包含迭代器所有值的 vector。collect 可以被用来创建很多类型的集合，所以这里显式注明 args 的类型来指定我们需要一个字符串 vector。虽然在 Rust 中我们很少会需要注明类型，然而 collect 是一个经常需要注明类型的函数，因为 Rust 不能推断出你想要什么类型的集合。

最后，我们使用调试宏打印出 vector。让我们尝试分别用两种方式（不包含参数和包含参数）运行代码：

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
]
```

```
$ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]
```

注意 vector 的第一个值是 "target/debug/minigrep"，它是我们二进制文件的名称。这与 C 中的参数列表的行为相匹配，让程序使用在执行时调用它们的名称。如果要在消息中打印它或者根据用于调用程序的命令行别名更改程序的行为，通常可以方便地访问程序名称，不过考虑到本章的目的，我们将忽略它并只保存所需的两个参数。

### 保存参数值

目前程序可以访问指定为命令行参数的值。现在需要将这两个参数的值保存进变量这样就可以在程序的余下部分使用这些值了。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
```

正如之前打印出 vector 时所所看到的，程序的名称占据了 vector 的第一个值 `args[0]`，所以我们从索引为 1 的参数开始。minigrep 获取的第一个参数是需要搜索的字符串，所以将其将第一个参数的引用存放在变量 query 中。第二个参数将是文件路径，所以将第二个参数的引用放入变量 file_path 中。

我们将临时打印出这些变量的值来证明代码如我们期望的那样工作。使用参数 test 和 sample.txt 再次运行这个程序：

```
$ cargo run -- test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```

好的，它可以工作！我们将所需的参数值保存进了对应的变量中。之后会增加一些错误处理来应对类似用户没有提供参数的情况，不过现在我们将忽略它们并开始增加读取文件功能。

## 读取文件

现在我们要增加读取由 file_path 命令行参数指定的文件的功能。首先，需要一个用来测试的示例文件：我们会用一个拥有多行少量文本且有一些重复单词的文件。示例是一首艾米莉·狄金森（Emily Dickinson）的诗，它正适合这个工作！在项目根目录创建一个文件 poem.txt，并输入诗 "I'm nobody! Who are you?"：

文件名：poem.txt

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

创建完这个文件之后，修改 src/main.rs 并增加如所示的打开文件的代码：

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

首先，我们增加了一个 `use` 语句来引入标准库中的相关部分：我们需要 `std::fs` 来处理文件。

在 main 中新增了一行语句：`fs::read_to_string` 接受 `file_path`，打开文件，接着返回包含其内容的 `std::io::Result<String>`。

在这些代码之后，我们再次增加了临时的 `println!` 打印出读取文件之后 `contents` 的值，这样就可以检查目前为止的程序能否工作。

尝试运行这些代码，随意指定一个字符串作为第一个命令行参数（因为还未实现搜索功能的部分）而将 poem.txt 文件将作为第二个参数：

```
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

好的！代码读取并打印出了文件的内容。虽然它还有一些瑕疵：此时 main 函数有着多个职能，通常函数只负责一个功能的话会更简洁并易于维护。另一个问题是没有尽可能的处理错误。虽然我们的程序还很小，这些瑕疵并不是什么大问题，不过随着程序功能的丰富，将会越来越难以用简单的方法修复它们。在开发程序时，及早开始重构是一个最佳实践，因为重构少量代码时要容易的多，所以让我们现在就开始吧。

## 重构改进模块性和错误处理

为了改善我们的程序这里有四个问题需要修复，而且它们都与程序的组织方式和如何处理潜在错误有关。第一，main 现在进行了两个任务：它解析了参数并打开了文件。对于一个这样的小函数，这并不是一个大问题。然而如果 main 中的功能持续增加，main 函数处理的独立任务也会增加。当函数承担了更多责任，它就更难以推导，更难以测试，并且更难以在不破坏其他部分的情况下做出修改。最好能分离出功能以便每个函数就负责一个任务。

这同时也关系到第二个问题：query 和 file_path 是程序中的配置变量，而像 contents 则用来执行程序逻辑。随着 main 函数的增长，就需要引入更多的变量到作用域中，而当作用域中有更多的变量时，将更难以追踪每个变量的目的。最好能将配置变量组织进一个结构，这样就能使它们的目的更明确了。

第三个问题是如果打开文件失败我们使用 expect 来打印出错误信息，不过这个错误信息只是说 Should have been able to read the file。读取文件失败的原因有多种：例如文件不存在，或者没有打开此文件的权限。目前，无论处于何种情况，我们只是打印出“文件读取出现错误”的信息，这并没有给予使用者具体的信息！

第四，我们不停地使用 expect 来处理不同的错误，如果用户没有指定足够的参数来运行程序，他们会从 Rust 得到 index out of bounds 错误，而这并不能明确地解释问题。如果所有的错误处理都位于一处，这样将来的维护者在需要修改错误处理逻辑时就只需要考虑这一处代码。将所有的错误处理都放在一处也有助于确保我们打印的错误信息对终端用户来说是有意义的。

让我们通过重构项目来解决这些问题。

### 项目拆分

main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导。这些过程有如下步骤：

- 将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
- 当命令行解析逻辑比较小时，可以保留在 main.rs 中。
- 当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

经过这些过程之后保留在 main 函数中的责任应该被限制为：

- 使用参数值调用命令行解析逻辑
- 设置任何其他的配置
- 调用 lib.rs 中的 run 函数
- 如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试它们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。

### 提取参数解析器

首先，我们将解析参数的功能提取到一个 main 将会调用的函数中，为将命令行解析逻辑移动到 src/lib.rs 中做准备。调用了新函数 parse_config。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

我们仍然将命令行参数收集进一个 vector，不过不同于在 main 函数中将索引 1 的参数值赋值给变量 query 和将索引 2 的值赋值给变量 file_path，我们将整个 vector 传递给 parse_config 函数。接着 parse_config 函数将包含决定哪个参数该放入哪个变量的逻辑，并将这些值返回到 main。仍然在 main 中创建变量 query 和 file_path，不过 main 不再负责处理命令行参数与变量如何对应。

这对重构我们这小程序可能有点大材小用，不过我们将采用小的、增量的步骤进行重构。在做出这些改变之后，再次运行程序并验证参数解析是否仍然正常。经常验证你的进展是一个好习惯，这样在遇到问题时能帮助你定位问题的成因。

### 组合配置值

我们可以采取另一个小的步骤来进一步改善这个函数。现在函数返回一个元组，不过立刻又将元组拆成了独立的部分。这是一个我们可能没有进行正确抽象的信号。

另一个表明还有改进空间的迹象是 parse_config 名称的 config 部分，它暗示了我们返回的两个值是相关的并都是一个配置值的一部分。目前除了将这两个值组合进元组之外并没有表达这个数据结构的意义：我们可以将这两个值放入一个结构体并给每个字段一个有意义的名字。这会让未来的维护者更容易理解不同的值如何相互关联以及它们的目的。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

新定义的结构体 `Config` 中包含字段 query 和 file_path。 parse_config 的签名表明它现在返回一个 Config 值。在之前的 parse_config 函数体中，我们返回了引用 args 中 String 值的字符串 slice，现在我们定义 `Config` 来包含拥有所有权的 String 值。main 中的 args 变量是参数值的所有者并只允许 parse_config 函数借用它们，这意味着如果 Config 尝试获取 args 中值的所有权将违反 Rust 的借用规则。

还有许多不同的方式可以处理 String 的数据，而最简单但有些不太高效的方式是调用这些值的 `clone` 方法。这会生成 Config 实例可以拥有的数据的完整拷贝，不过会比储存字符串数据的引用消耗更多的时间和内存。不过拷贝数据使得代码显得更加直白因为无需管理引用的生命周期，所以在这种情况下牺牲一小部分性能来换取简洁性的取舍是值得的。

> **使用 clone 的权衡取舍**
>
> 由于其运行时消耗，许多 Rustacean 之间有一个趋势是倾向于避免使用 clone 来解决所有权问题。在关于迭代器的章中，我们将会学习如何更有效率的处理这种情况，不过现在，复制一些字符串来取得进展是没有问题的，因为只会进行一次这样的拷贝，而且文件路径和要搜索的字符串都比较短。在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些。随着你对 Rust 更加熟练，将能更轻松的直奔合适的方法，不过现在调用 clone 是完全可以接受的。

我们更新 main 将 parse_config 返回的 Config 实例放入变量 config 中，并将之前分别使用 query 和 file_path 变量的代码更新为现在的使用 Config 结构体的字段的代码。

现在代码更明确的表现了我们的意图，query 和 file_path 是相关联的并且它们的目的是配置程序如何工作。任何使用这些值的代码就知道在 config 实例中对应目的的字段名中寻找它们。

### 使用构造函数

目前为止，我们将负责解析命令行参数的逻辑从 main 提取到了 parse_config 函数中，这有助于我们看清值 query 和 file_path 是相互关联的并应该在代码中表现这种关系。接着我们增加了 Config 结构体来描述 query 和 file_path 的相关性，并能够从 parse_config 函数中将这些值的名称作为结构体字段名称返回。

所以现在 parse_config 函数的目的是创建一个 Config 实例，我们可以将 parse_config 从一个普通函数变为一个叫做 new 的与结构体关联的函数。做出这个改变使得代码更符合习惯：可以像标准库中的 String 调用 String::new 来创建一个该类型的实例那样，将 parse_config 变为一个与 Config 关联的 new 函数。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

这里将 main 中调用 parse_config 的地方更新为调用 `Config::new`。我们将 parse_config 的名字改为 new 并将其移动到 impl 块中，这使得 new 函数与 Config 相关联。再次尝试编译并确保它可以工作。

### 修复错误处理

现在我们开始修复错误处理。回忆一下之前提到过如果 args vector 包含少于 3 个项并尝试访问 vector 中索引 1 或索引 2 的值会造成程序 panic。尝试不带任何参数运行程序；这将看起来像这样：

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at src/main.rs:27:21:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

index out of bounds: the len is 1 but the index is 1 是一个针对程序员的错误信息，然而这并不能真正帮助终端用户理解发生了什么和他们应该做什么。现在就让我们修复它吧。

**改善错误信息**

在 new 函数中增加了一个检查在访问索引 1 和 2 之前检查 slice 是否足够长。如果 slice 不够长，程序会打印一个更好的错误信息并 panic:

```rust
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // --snip--
```

Guess::new 函数，那里如果 value 参数超出了有效值的范围就调用 panic!。不同于检查值的范围，这里检查 args 的长度至少是 3，而函数的剩余部分则可以在假设这个条件成立的基础上运行。如果 args 少于 3 个项，则这个条件将为真，并调用 panic! 立即终止程序。

有了 new 中这几行额外的代码，再次不带任何参数运行程序并看看现在错误看起来像什么：

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at src/main.rs:26:13:
not enough arguments
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这个输出就好多了，现在有了一个合理的错误信息。然而，还是有一堆额外的信息我们不希望提供给用户。所以在这里使用的技术可能不是最好的；panic! 的调用更趋向于程序上的问题而不是使用上的问题。相反我们可以使用第九章学习的另一个技术 -- 返回一个可以表明成功或错误的 `Result`。

**从 new 中返回 Result 而不是调用 panic!**

我们可以选择返回一个 Result 值，它在成功时会包含一个 Config 的实例，而在错误时会描述问题。我们还将把函数名从 `new` 改为 `build`，因为许多程序员希望 new 函数永远不会失败。当 Config::new 与 main 交流时，可以使用 Result 类型来表明这里存在问题。接着修改 main 将 Err 成员转换为对用户更友好的错误，而不是 panic! 调用产生的关于 thread 'main' 和 RUST_BACKTRACE 的文本。

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

现在 build 函数返回一个 Result，在成功时带有一个 Config 实例而在出现错误时带有一个 `&'static str`。 &'static str 是字符串字面值的类型，也是目前的错误信息。

build 函数体中有两处修改：当没有足够参数时不再调用 panic!，而是返回 `Err` 值。同时我们将 Config 返回值包装进 Ok 成员中。这些修改使得函数符合其新的类型签名。

通过让 Config::build 返回一个 Err 值，这就允许 main 函数处理 build 函数返回的 Result 值并在出现错误的情况更明确的结束进程。

**调用 Config::build 并处理错误**

为了处理错误情况并打印一个对用户友好的信息，我们需要更新 main 函数来处理现在 `Config::build` 返回的 `Result`。另外还需要手动实现原先由 panic!负责的工作，即以非零错误码退出命令行工具的工作。非零的退出状态是一个惯例信号，用来告诉调用程序的进程：该程序以错误状态退出了。

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}
```

在上面的示例中，使用了一个之前没有详细说明的方法：`unwrap_or_else`，它定义于标准库的 `Result<T, E>` 上。使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。然而，当其值是 Err 时，该方法会调用一个 **闭包**（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。 unwrap_or_else 会将 Err 的内部值，也就是 not enough arguments 静态字符串的情况，传递给闭包中位于两道竖线间的参数 err。闭包中的代码在其运行时可以使用这个 err 值。

我们新增了一个 use 行来从标准库中导入 `process`。在错误的情况闭包中将被运行的代码只有两行：我们打印出了 `err` 值，接着调用了 `std::process::exit`。process::exit 会立即停止程序并将传递给它的数字作为退出状态码。这类似于使用的基于 panic! 的错误处理，除了不会再得到所有的额外输出了。让我们试试：

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

### 简化 mian 函数

现在我们完成了配置解析的重构：让我们转向程序的逻辑。正如 “二进制项目的关注分离” 部分所展开的讨论，我们将提取一个叫做 run 的函数来存放目前 main 函数中不属于设置配置或处理错误的所有逻辑。一旦完成这些，main 函数将简明得足以通过观察来验证，而我们将能够为所有其他逻辑编写测试。

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// --snip--
```

现在 `run` 函数包含了 main 中从读取文件开始的剩余的所有逻辑。run 函数获取一个 `Config` 实例作为参数。

**从 run 函数中返回错误**

通过将剩余的逻辑分离进 run 函数而不是留在 main 中，就可以像 Config::build 那样改进错误处理。不再通过 expect 允许程序 panic，run 函数将会在出错时返回一个 `Result<T, E>`。这让我们进一步以一种对用户友好的方式统一 main 中的错误处理。

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

这里我们做出了三个明显的修改。首先，将 run 函数的返回类型变为 `Result<(), Box<dyn Error>>`。之前这个函数返回 unit 类型 ()，现在它仍然保持作为 Ok 时的返回值。

对于错误类型，使用了 trait 对象 Box<dyn Error>（在开头使用了 use 语句将 std::error::Error 引入作用域）。目前只需知道 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。

第二个改变是去掉了 `expect` 调用并替换为 `?`。不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它。

第三个修改是现在成功时这个函数会返回一个 `Ok` 值。因为 run 函数签名中声明成功类型返回值是 `()`，这意味着需要将 unit 类型值包装进 Ok 值中。`Ok(())` 一开始看起来有点奇怪，不过这样使用 `()` 是惯用的做法，表明调用 run 函数只是为了它的副作用；函数并没有返回什么有意义的值。

上述代码能够编译，不过会有一个警告：

```
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
warning: unused `Result` that must be used
  --> src/main.rs:19:5
   |
19 |     run(config);
   |     ^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
19 |     let _ = run(config);
   |     +++++++

warning: `minigrep` (bin "minigrep") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Rust 提示我们的代码忽略了 Result 值，它可能表明这里存在一个错误。但我们却没有检查这里是否有一个错误，而编译器提醒我们这里应该有一些错误处理代码！现在就让我们修正这个问题。

**处理 main 中 run 返回的错误**

我们将检查错误并使用类似 Config::build 处理错误的技术来处理它们，不过有一些细微的不同：

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

我们使用 `if let` 来检查 run 是否返回一个 `Err` 值，不同于 unwrap_or_else，并在出错时调用 process::exit(1)。run 并不返回像 Config::build 返回的 Config 实例那样需要 unwrap 的值。因为 run 在成功时返回 ()，而我们只关心检测错误，所以并不需要 unwrap_or_else 来返回未封装的值，因为它只会是 ()。

不过两个例子中 if let 和 unwrap_or_else 的函数体都一样：打印出错误并退出。

### 拆分代码倒 crate

现在我们的 minigrep 项目看起来好多了！现在我们将要拆分 src/main.rs 并将一些代码放入 src/lib.rs，这样就能测试它们并拥有一个含有更少功能的 main 函数。

让我们将所有不是 main 函数的代码从 src/main.rs 移动到新文件 src/lib.rs 中：

- `run` 函数定义
- 相关的 `use` 语句
- `Config` 的定义
- `Config::build` 函数定义

现在 src/lib.rs 的内容应该看起来:

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```

这里使用了公有的 pub 关键字：在 Config、其字段和其 build 方法，以及 run 函数上。现在我们有了一个拥有可以测试的公有 API 的库 crate 了。

现在需要在 src/main.rs 中将移动到 src/lib.rs 的代码引入二进制 crate 的作用域中:

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```

我们添加了一行 `use minigrep::Config`，它将 Config 类型引入作用域，并使用 crate 名称作为 run 函数的前缀。通过这些重构，所有功能应该能够联系在一起并运行了。运行 cargo run 来确保一切都正确的衔接在一起。

哇哦！我们做了大量的工作，不过我们为将来的成功打下了基础。现在处理错误将更容易，同时代码也更加模块化。从现在开始几乎所有的工作都将在 src/lib.rs 中进行。

让我们利用这些新创建的模块的优势来进行一些在旧代码中难以展开的工作，这些工作在新代码中非常容易实现，那就是：编写测试！

## 测试驱动开发 （TDD）

现在我们将逻辑提取到了 src/lib.rs 并将所有的参数解析和错误处理留在了 src/main.rs 中，为代码的核心功能编写测试将更加容易。我们可以直接使用多种参数调用函数并检查返回值而无需从命令行运行二进制文件了。

在这一部分，我们将遵循测试驱动开发（Test Driven Development, TDD）的模式来逐步增加 minigrep 的搜索逻辑。它遵循如下步骤：

1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
2. 编写或修改足够的代码来使新的测试通过。
3. 重构刚刚增加或修改的代码，并确保测试仍然能通过。
4. 从步骤 1 开始重复！

虽然这只是众多编写软件的方法之一，不过 TDD 有助于驱动代码的设计。在编写能使测试通过的代码之前编写测试有助于在开发过程中保持高测试覆盖率。

我们将测试驱动实现实际在文件内容中搜索查询字符串并返回匹配的行示例的功能。我们将在一个叫做 search 的函数中增加这些功能。

### 编写失败测试

去掉 src/lib.rs 和 src/main.rs 中用于检查程序行为的 println! 语句，因为不再真正需要它们了。接着我们会增加一个 test 模块和一个测试函数。测试函数指定了 search 函数期望拥有的行为：它会获取一个需要查询的字符串和用来查询的文本，并只会返回包含请求的文本行。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick there.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

这里选择使用 "duct" 作为这个测试中需要搜索的字符串。用来搜索的文本有三行，其中只有一行包含 "duct"。（注意双引号之后的反斜杠，这告诉 Rust 不要在字符串字面值内容的开头加入换行符）我们断言 search 函数的返回值只包含期望的那一行。

我们还不能运行这个测试并看到它失败，因为它甚至都还不能编译：`search` 函数还不存在呢！根据 TDD 的原则，我们将增加足够的代码来使其能够编译：一个总是会返回空 vector 的 search 函数定义。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

注意需要在 `search` 的签名中定义一个显式生命周期 `'a` 并用于 `contents` 参数和返回值。生命周期参数指定哪个参数的生命周期与返回值的生命周期相关联。在这个例子中，我们表明返回的 vector 中应该包含引用参数 contents（而不是参数query）slice 的字符串 slice。

换句话说，我们告诉 Rust 函数 `search` 返回的数据将与 search 函数中的参数 `contents` 的数据存在的一样久。这是非常重要的！为了使这个引用有效那么 **被** slice 引用的数据也需要保持有效；如果编译器认为我们是在创建 query 而不是 contents 的字符串 slice，那么安全检查将是不正确的。

如果尝试不用生命周期编译的话，我们将得到如下错误：

```
$ cargo build
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
error[E0106]: missing lifetime specifier
  --> src/lib.rs:28:51
   |
28 | pub fn search(query: &str, contents: &str) -> Vec<&str> {
   |                      ----            ----         ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
help: consider introducing a named lifetime parameter
   |
28 | pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
   |              ++++         ++                 ++              ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `minigrep` (lib) due to 1 previous error
```

Rust 不可能知道我们需要的是哪一个参数，所以需要告诉它。因为参数 contents 包含了所有的文本而且我们希望返回匹配的那部分文本，所以我们知道 contents 是应该要使用生命周期语法来与返回值相关联的参数。

现在运行测试：

```
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.97s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'tests::one_result' panicked at src/lib.rs:44:9:
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

好的，测试失败了，这正是我们所期望的。修改代码来让测试通过吧！

### 修正代码

目前测试之所以会失败是因为我们总是返回一个空的 vector。为了修复并实现 search，我们的程序需要遵循如下步骤：

- 遍历内容的每一行文本。
- 查看这一行是否包含要搜索的字符串。
- 如果有，将这一行加入列表返回值中。
- 如果没有，什么也不做。
- 返回匹配到的结果列表

让我们一步一步的来，从遍历每行开始。

**使用 lines 方法遍历每一行**

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

lines 方法返回一个迭代器，使用了一个 for 循环和迭代器在一个集合的每一项上运行了一些代码。

**用查询字符串搜索每一行**

接下来将会增加检查当前行是否包含查询字符串的功能。幸运的是，字符串类型为此也有一个叫做 contains 的实用方法！在 search 函数中加入 contains 方法调用。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // 对文本行进行操作
        }
    }
}
```

**存储匹配的行**

为了完成这个函数，我们还需要一个方法来存储包含查询字符串的行。为此可以在 for 循环之前创建一个可变的 vector 并调用 push 方法在 vector 中存放一个 line。在 for 循环之后，返回这个 vector：

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

现在 search 函数应该返回只包含 query 的那些行，而测试应该会通过。让我们运行测试：

```
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.22s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

测试通过了，它可以工作了！

现在正是可以考虑重构的时机，在保证测试通过，保持功能不变的前提下重构 search 函数。

**在 run 函数中使用 search 函数**

现在 `search` 函数是可以工作并测试通过了的，我们需要实际在 `run` 函数中调用 search。需要将 config.query 值和 run 从文件中读取的 contents 传递给 search 函数。接着 run 会打印出 search 返回的每一行：

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

这里仍然使用了 for 循环获取了 search 返回的每一行并打印出来。

现在整个程序应该可以工作了！让我们试一试，首先使用一个只会在艾米莉·狄金森的诗中返回一行的单词 “frog”：

```
$ cargo run -- frog poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/minigrep frog poem.txt`
How public, like a frog
```

好的！现在试试一个会匹配多行的单词，比如 “body”：

```
$ cargo run -- body poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep body poem.txt`
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

最后，让我们确保搜索一个在诗中哪里都没有的单词时不会得到任何行，比如 "monomorphization"：

```
$ cargo run -- monomorphization poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep monomorphization poem.txt`
```

非常好！我们创建了一个属于自己的迷你版经典工具，并学习了很多如何组织程序的知识。我们还学习了一些文件输入输出、生命周期、测试和命令行解析的内容。

## 处理环境变量

我们将增加一个额外的功能来改进 minigrep：用户可以通过设置环境变量来设置搜索是否是大小写敏感的。当然，我们也可以将其设计为一个命令行参数并要求用户每次需要时都加上它，不过在这里我们将使用环境变量。这允许用户设置环境变量一次之后在整个终端会话中所有的搜索都将是大小写不敏感的。

首先我们希望增加一个新函数 `search_case_insensitive`，并将会在环境变量有值时调用它。这里将继续遵循 TDD 过程，其第一步是再次编写一个失败测试。我们将为新的大小写不敏感搜索函数新增一个测试函数，并将老的测试函数从 one_result 改名为 case_sensitive 来更清楚的表明这两个测试的区别：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick there.\nDuct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick there.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

注意我们也改变了老测试中 contents 的值。还新增了一个含有文本 "Duct tape." 的行，它有一个大写的 D，这在大小写敏感搜索时不应该匹配 "duct"。我们修改这个测试以确保不会意外破坏已经实现的大小写敏感搜索功能；这个测试现在应该能通过并在处理大小写不敏感搜索时应该能一直通过。

大小写 不敏感 搜索的新测试使用 "rUsT" 作为其查询字符串。在我们将要增加的 search_case_insensitive 函数中，"rUsT" 查询应该包含带有一个大写 R 的 "Rust:" 还有 "Trust me." 这两行，即便它们与查询的大小写都不同。这个测试现在不能编译，因为还没有定义 search_case_insensitive 函数。请随意增加一个总是返回空 vector 的骨架实现，为了使测试通过编译并失败时所做的那样。

**实现 search_case_insensitive 函数**

search_case_insensitive 函数，将与 search 函数基本相同。唯一的区别是它会将 query 变量和每一 line 都变为小写，这样不管输入参数是大写还是小写，在检查该行是否包含查询字符串时都会是小写。

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

首先我们将 `query` 字符串转换为小写，并将其覆盖到同名的变量中。对查询字符串调用 `to_lowercase` 是必需的，这样不管用户的查询是 "rust"、"RUST"、"Rust" 或者 "rUsT"，我们都将其当作 "rust" 处理并对大小写不敏感。虽然 to_lowercase 可以处理基本的 Unicode，但它不是 100% 准确。如果编写真实的程序的话，我们还需多做一些工作，不过这一部分是关于环境变量而不是 Unicode 的，所以这样就够了。

注意 query 现在是一个 String 而不是字符串 slice，因为调用 to_lowercase 是在创建新数据，而不是引用现有数据。如果查询字符串是 "rUsT"，这个字符串 slice 并不包含可供我们使用的小写的 u 或 t，所以必需分配一个包含 "rust" 的新 String。现在当我们将 query 作为一个参数传递给 contains 方法时，需要增加一个 & 因为 contains 的签名被定义为获取一个字符串 slice。

接下来我们对每一 line 都调用 to_lowercase 将其转为小写。现在我们将 line 和 query 都转换成了小写，这样就可以不管查询的大小写进行匹配了。

让我们看看这个实现能否通过测试：

```
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

好的！现在，让我们在 run 函数中实际调用新 `search_case_insensitive` 函数。首先，我们将在 Config 结构体中增加一个配置项来切换大小写敏感和大小写不敏感搜索。增加这些字段会导致编译错误，因为我们还没有在任何地方初始化这些字段：

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```

这里增加了 `ignore_case` 字符来存放一个布尔值。接着我们需要 run 函数检查 case_sensitive 字段的值并使用它来决定是否调用 search 函数或 search_case_insensitive 函数。

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

最后需要实际检查环境变量。处理环境变量的函数位于标准库的 `env` 模块中，所以我们需要在 src/lib.rs 的开头将这个模块引入作用域中。接着使用 env 模块的 `var` 方法来检查一个叫做 `IGNORE_CASE` 的环境变量：

```rust
use std::env;
// --snip--

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

这里创建了一个新变量 `ignore_case`。为了设置它的值，需要调用 `env::var` 函数并传递我们需要寻找的环境变量名称，`IGNORE_CASE`。env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员。

我们使用 Result 的 `is_ok` 方法来检查环境变量是否被设置，这也就意味着我们 **需要** 进行一个大小写不敏感的搜索。如果IGNORE_CASE 环境变量没有被设置为任何值，is_ok 会返回 false 并将进行大小写敏感的搜索。我们并不关心环境变量所设置的 值，只关心它是否被设置了，所以检查 is_ok 而不是 unwrap、expect 或任何我们已经见过的 Result 的方法。

我们将变量 ignore_case 的值传递给 Config 实例，这样 run 函数可以读取其值并决定是否调用 `search` 或者 `search_case_insensitive`。

```
$ cargo run -- to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

看起来程序仍然能够工作！现在将 IGNORE_CASE 设置为 1 并仍使用相同的查询 to。

```bash
$ IGNORE_CASE=1 cargo run to poem.txt
```

如果你使用 PowerShell，则需要用两个命令来分别设置环境变量并运行程序：

```powershell
PS> $Env:IGNORE_CASE=1; cargo run to poem.txt
```

而这会让 IGNORE_CASE 的效果在当前 shell 会话中持续生效。可以通过 Remove-Item 命令来取消设置：

```powershell
PS> Remove-Item Env:IGNORE_CASE
```

这回应该得到包含可能有大写字母的 “to” 的行：

```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

好极了，我们也得到了包含 “To” 的行！现在 minigrep 程序可以通过环境变量控制进行大小写不敏感搜索了。现在你知道了如何管理由命令行参数或环境变量设置的选项了！

一些程序允许对相同配置同时使用参数 **和** 环境变量。在这种情况下，程序来决定参数和环境变量的优先级。

## 输出重定向

目前为止，我们将所有的输出都通过 println! 写到了终端。大部分终端都提供了两种输出：**标准输出**（standard output，stdout）对应一般信息，**标准错误**（standard error，stderr）则用于错误信息。这种区别允许用户选择将程序正常输出定向到一个文件中并仍将错误信息打印到屏幕上。

但是 println! 宏只能够打印到标准输出，所以我们必须使用其他方法来打印到标准错误。

**检查错误应该写入何处**

首先，让我们观察一下目前 minigrep 打印的所有内容是如何被写入标准输出的，包括那些应该被写入标准错误的错误信息。可以通过将标准输出流重定向到一个文件同时有意产生一个错误来做到这一点。我们没有重定向标准错误流，所以任何发送到标准错误的内容将会继续显示在屏幕上。

命令行程序被期望将错误信息发送到标准错误流，这样即便选择将标准输出流重定向到文件中时仍然能看到错误信息。目前我们的程序并不符合期望；相反我们将看到它将错误信息输出保存到了文件中！

我们通过 > 和文件路径 output.txt 来运行程序，我们期望重定向标准输出流到该文件中。在这里，我们没有传递任何参数，所以会产生一个错误：

```
$ cargo run > output.txt
```

`>` 语法告诉 shell 将标准输出的内容写入到 output.txt 文件中而不是屏幕上。我们并没有看到期望的错误信息打印到屏幕上，所以这意味着它一定被写入了文件中。如下是 output.txt 所包含的：

```
Problem parsing arguments: not enough arguments
```

是的，错误信息被打印到了标准输出中。像这样的错误信息被打印到标准错误中将会有用得多，将使得只有成功运行所产生的输出才会写入文件。我们接下来就修改。

**将错误打印到标准错误**

得益于本章早些时候的重构，所有打印错误信息的代码都位于 main 一个函数中。标准库提供了 eprintln! 宏来打印到标准错误流，所以将两个调用 println! 打印错误信息的位置替换为 eprintln!：

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

现在我们再次尝试用同样的方式运行程序，不使用任何参数并通过 `>` 重定向标准输出：

```
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

现在我们看到了屏幕上的错误信息，同时 output.txt 里什么也没有，这正是命令行程序所期望的行为。

如果使用不会造成错误的参数再次运行程序，不过仍然将标准输出重定向到一个文件，像这样：

```
$ cargo run -- to poem.txt > output.txt
```

我们并不会在终端看到任何输出，同时 output.txt 将会包含其结果：

文件名：output.txt

```
Are you nobody, too?
How dreary to be somebody!
```

这一部分展示了现在我们适当的使用了成功时产生的标准输出和错误时产生的标准错误。
