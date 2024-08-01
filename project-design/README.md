# Rust project design

当你编写大型程序时，组织你的代码显得尤为重要。通过对相关功能进行分组和划分不同功能的代码，你可以清楚在哪里可以找到实现了特定功能的代码，以及在哪里可以改变一个功能的工作方式。

到目前为止，我们编写的程序都在一个文件的一个模块中。伴随着项目的增长，你应该通过将代码分解为多个模块和多个文件来组织代码。一个包可以包含多个二进制 crate 项和一个可选的 crate 库。伴随着包的增长，你可以将包中的部分代码提取出来，做成独立的 crate，这些 crate 则作为外部依赖项

Rust 有许多功能可以让你管理代码的组织，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。这些功能，有时被统称为 “模块系统（the module system）”，包括：

- **包**（_Packages_）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
- **Crates** ：一个模块的树形结构，它形成了库或二进制项目。
- **模块**（_Modules_）和 **use**：允许你控制作用域和路径的私有性。
- **路径**（_path_）：一个命名例如结构体、函数或模块等项的方式。

crate 是 Rust 在编译时最小的代码单位。如果你用 rustc 而不是 cargo 来编译一个文件（第一章我们这么做过），编译器还是会将那个文件认作一个 crate。crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译。

crate 有两种形式：二进制项和库。二进制项 可以被编译为可执行程序，比如一个命令行程序或者一个 web server。它们必须有一个 main 函数来定义当程序被执行的时候所需要做的事情。目前我们所创建的 crate 都是二进制项。

库 并没有 main 函数，它们也不会编译为可执行程序，它们提供一些诸如函数之类的东西，使其他项目也能使用这些东西。比如之前用到的 rand crate 就提供了生成随机数的东西。大多数时间 Rustaceans 说的 crate 指的都是库，这与其他编程语言中 library 概念一致。

## packge

**包**（package）是提供一系列功能的一个或者多个 crate。一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。Cargo 就是一个包含构建你代码的二进制项的包。Cargo 也包含这些二进制项所依赖的库。其他项目也能用 Cargo 库来实现与 Cargo 命令行程序一样的逻辑。

包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。

让我们来看看创建包的时候会发生什么。首先，我们输入命令 `cargo new`：

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

运行了这条命令后，我们先用 ls 来看看 Cargo 给我们创建了什么，Cargo 会给我们的包创建一个 Cargo.toml 文件。查看 Cargo.toml 的内容，会发现并没有提到 src/main.rs，因为 Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。同样的，Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

在此，我们有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 `my-project` 的二进制 crate。如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。

## 模块小抄

这里提供一个简单的参考，用来解释模块、路径、use关键词和pub关键词如何在编译器中工作，以及大部分开发者如何组织他们的代码。

- **从 crate 根节点开始**: 当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是_src/lib.rs_，对于一个二进制 crate 而言是_src/main.rs_）中寻找需要被编译的代码。
- **声明模块**: 在 crate 根文件中，你可以声明一个新模块；比如，你用`mod garden;`声明了一个叫做`garden`的模块。编译器会在下列路径中寻找模块代码：
    - 内联，在大括号中，当`mod garden`后方不是一个分号而是一个大括号
    - 在文件 _src/garden.rs_
    - 在文件 _src/garden/mod.rs_
- **声明子模块**: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在_src/garden.rs_中定义了`mod vegetables;`。编译器会在以父模块命名的目录中寻找子模块代码：
    - 内联，在大括号中，当`mod vegetables`后方不是一个分号而是一个大括号
    - 在文件 _src/garden/vegetables.rs_
    - 在文件 _src/garden/vegetables/mod.rs_
- **模块中的代码路径**: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的`Asparagus`类型可以在`crate::garden::vegetables::Asparagus`被找到。
- **私有 vs 公用**: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用`pub mod`替代`mod`。为了使一个公用模块内部的成员公用，应当在声明前使用`pub`。
- **`use` 关键字**: 在一个作用域内，`use`关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用`crate::garden::vegetables::Asparagus`的作用域，你可以通过 `use crate::garden::vegetables::Asparagus;`创建一个快捷方式，然后你就可以在作用域中只写`Asparagus`来使用该类型。

我们创建一个名为backyard的二进制 crate 来说明这些规则。该 crate 的路径同样命名为backyard，该路径包含了这些文件和目录：

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

这个例子中的 crate 根文件是src/main.rs，该文件包括了：

文件名：src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

`pub mod garden;`行告诉编译器应该包含在src/garden.rs文件中发现的代码：

文件名：src/garden.rs

```rust
pub mod vegetables;
```

在此处， pub mod vegetables;意味着在src/garden/vegetables.rs中的代码也应该被包括。这些代码是：

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

现在让我们深入了解这些规则的细节并在实际中演示它们！

## 组织模块文件

_模块_ 让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性。因为一个模块中的代码默认是私有的，所以还可以利用模块控制项的 _私有性_。私有项是不可为外部使用的内在详细实现。我们也可以将模块和它其中的项标记为公开的，这样，外部代码就可以使用并依赖与它们。

在餐饮业，餐馆中会有一些地方被称之为 _前台_（_front of house_），还有另外一些地方被称之为 _后台_（_back of house_）。前台是招待顾客的地方，在这里，店主可以为顾客安排座位，服务员接受顾客下单和付款，调酒师会制作饮品。后台则是由厨师工作的厨房，洗碗工的工作地点，以及经理做行政工作的地方组成。

我们可以将函数放置到嵌套的模块中，来使我们的 crate 结构与实际的餐厅结构相同。通过执行 cargo new --lib restaurant，来创建一个新的名为 restaurant 的库。

文件名：src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

我们定义一个模块，是以 `mod` 关键字为起始，然后指定模块的名字（本例中叫做 front_of_house），并且用花括号包围模块的主体。在模块内，我们还可以定义其他的模块，就像本例中的 hosting 和 serving 模块。模块还可以保存一些定义的其他项，比如结构体、枚举、常量、特性、或者函数。

通过使用模块，我们可以将相关的定义分组到一起，并指出它们为什么相关。程序员可以通过使用这段代码，更加容易地找到他们想要的定义，因为他们可以基于分组来对代码进行导航，而不需要阅读所有的定义。程序员向这段代码中添加一个新的功能时，他们也会知道代码应该放置在何处，可以保持程序的组织性。

在前面我们提到了，src/main.rs 和 src/lib.rs 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

这个树展示了一些模块是如何被嵌入到另一个模块的（例如，hosting 嵌套在 front_of_house 中）。这个树还展示了一些模块是互为 _兄弟_（siblings）的，这意味着它们定义在同一模块中（hosting 和 serving 被一起定义在 front_of_house 中）。继续沿用家庭关系的比喻，如果一个模块 A 被包含在模块 B 中，我们将模块 A 称为模块 B 的 子（child），模块 B 则是模块 A 的 父（parent）。注意，整个模块树都植根于名为 crate 的隐式模块下。

这个模块树可能会令你想起电脑上文件系统的目录树；这是一个非常恰当的类比！就像文件系统的目录，你可以使用模块来组织你的代码。并且，就像目录中的文件，我们需要一种方法来找到模块。

## 引用模块

来看一下 Rust 如何在模块树中找到一个项的位置，我们使用路径的方式，就像在文件系统使用路径一样。为了调用一个函数，我们需要知道它的路径。

路径有两种形式：

- **绝对路径**（absolute path）是以 `crate` 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于当前 crate 的代码，则以字面值 crate 开头。
- **相对路径**（relative path）从当前模块开始，以 self、super 或定义在当前模块中的标识符开头。

绝对路径和相对路径都后跟一个或多个由双冒号（`::`）分割的标识符。

还是之前 `front_of_house` 的例子，假设我们希望调用 add_to_waitlist 函数。add_to_waitlist 函数的路径是什么？

我们在 crate 根定义了一个新函数 `eat_at_restaurant`，并在其中展示调用 add_to_waitlist 函数的两种方法。eat_at_restaurant 函数是我们 crate 库的一个公共 API，所以我们使用 pub 关键字来标记它。注意，这个例子无法编译通过，我们稍后会解释原因。

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

第一种方式，我们在 `eat_at_restaurant` 中调用 `add_to_waitlist` 函数，使用的是绝对路径。add_to_waitlist 函数与 eat_at_restaurant 被定义在同一 crate 中，这意味着我们可以使用 crate 关键字为起始的绝对路径。

在 crate 后面，我们持续地嵌入模块，直到我们找到 add_to_waitlist。你可以想象出一个相同结构的文件系统，我们通过指定路径 _/front_of_house/hosting/add_to_waitlist_ 来执行 add_to_waitlist 程序。我们使用 crate 从 crate 根开始就类似于在 shell 中使用 / 从文件系统根开始。

第二种方式，我们在 eat_at_restaurant 中调用 add_to_waitlist，使用的是相对路径。这个路径以 front_of_house 为起始，这个模块在模块树中，与 eat_at_restaurant 定义在同一层级。与之等价的文件系统路径就是 _front_of_house/hosting/add_to_waitlist_。以模块名开头意味着该路径是相对路径。

选择使用相对路径还是绝对路径，要取决于你的项目，也取决于你是更倾向于将项的定义代码与使用该项的代码分开来移动，还是一起移动。举一个例子，如果我们要将 front_of_house 模块和 eat_at_restaurant 函数一起移动到一个名为 customer_experience 的模块中，我们需要更新 add_to_waitlist 的绝对路径，但是相对路径还是可用的。然而，如果我们要将 eat_at_restaurant 函数单独移到一个名为 dining 的模块中，还是可以使用原本的绝对路径来调用 add_to_waitlist，但是相对路径必须要更新。我们更倾向于使用绝对路径，因为把代码定义和项调用各自独立地移动是更常见的。

尝试编译会发现以下错误：

```
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` (lib) due to 2 previous errors
```

错误信息说 hosting 模块是私有的。换句话说，我们拥有 hosting 模块和 add_to_waitlist 函数的正确路径，但是 Rust 不让我们使用，因为它不能访问私有片段。在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的。如果希望创建一个私有函数或结构体，你可以将其放入一个模块。

父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用它们父模块中的项。这是因为子模块封装并隐藏了它们的实现详情，但是子模块可以看到它们定义的上下文。继续拿餐馆作比喻，把私有性规则想象成餐馆的后台办公室：餐馆内的事务对餐厅顾客来说是不可知的，但办公室经理可以洞悉其经营的餐厅并在其中做任何事情。

Rust 选择以这种方式来实现模块系统功能，因此默认隐藏内部实现细节。这样一来，你就知道可以更改内部代码的哪些部分而不会破坏外部代码。不过 Rust 也确实提供了通过使用 `pub` 关键字来创建公共项，使子模块的内部部分暴露给上级模块。

## pub 关键字

使用 `pub` 关键字暴露路径，上面 `hosting` 模块是私有的。我们想让父模块中的 eat_at_restaurant 函数可以访问子模块中的 add_to_waitlist 函数，因此我们使用 pub 关键字来标记 hosting 模块:

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

现在代码可以编译通过了！为了了解为何增加 pub 关键字使得我们可以在 add_to_waitlist 中调用这些路径与私有性规则有关，让我们看看绝对路径和相对路径。

在绝对路径，我们从 crate 也就是 crate 根开始。crate 根中定义了 front_of_house 模块。虽然 front_of_house 模块不是公有的，不过因为 eat_at_restaurant 函数与 front_of_house 定义于同一模块中（即，eat_at_restaurant 和 front_of_house 是兄弟模块），我们可以从 eat_at_restaurant 中引用 front_of_house。接下来是使用 pub 标记的 hosting 模块。我们可以访问 hosting 的父模块，所以可以访问 hosting。最后，add_to_waitlist 函数被标记为 pub ，我们可以访问其父模块，所以这个函数调用是有效的！

在相对路径，其逻辑与绝对路径相同，除了第一步：不同于从 crate 根开始，路径从 front_of_house 开始。front_of_house 模块与 eat_at_restaurant 定义于同一模块，所以从 eat_at_restaurant 中开始定义的该模块相对路径是有效的。接下来因为 hosting 和 add_to_waitlist 被标记为 pub，路径其余的部分也是有效的，因此函数调用也是有效的！

如果你计划共享你的库 crate 以便其它项目可以使用你的代码，公有 API 将是决定 crate 用户如何与你代码交互的契约。

> **二进制和库 crate 包的最佳实践**
> 我们提到过包可以同时包含一个 src/main.rs 二进制 crate 根和一个 src/lib.rs 库 crate 根，并且这两个 crate 默认以包名来命名。通常，这种包含二进制 crate 和库 crate 的模式的包，在二进制 crate 中只有足够的代码来启动一个可执行文件，可执行文件调用库 crate 的代码。又因为库 crate 可以共享，这使得其它项目从包提供的大部分功能中受益。
>
> 模块树应该定义在 src/lib.rs 中。这样通过以包名开头的路径，公有项就可以在二进制 crate 中使用。二进制 crate 就完全变成了同其它 外部 crate 一样的库 crate 的用户：它只能使用公有 API。这有助于你设计一个好的 API；你不仅仅是作者，也是用户！

## super 关键字

我们可以通过在路径的开头使用 `super` ，从父模块开始构建相对路径，而不是从当前模块或者 crate 根开始。这类似以 `..` 语法开始一个文件系统路径。使用 super 允许我们引用父模块中的已知项，这使得重新组织模块树变得更容易 —— 当模块与父模块关联的很紧密，但某天父模块可能要移动到模块树的其它位置。

考虑以下示例，它模拟了厨师更正了一个错误订单，并亲自将其提供给客户的情况。back_of_house 模块中的定义的 `fix_incorrect_order` 函数通过指定的 super 起始的 `deliver_order` 路径，来调用父模块中的 deliver_order 函数：

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

使用以 super 开头的相对路径从父目录开始调用函数, fix_incorrect_order 函数在 back_of_house 模块中，所以我们可以使用 super 进入 back_of_house 父模块，也就是本例中的 crate 根。在这里，我们可以找到 deliver_order。成功！我们认为 back_of_house 模块和 deliver_order 函数之间可能具有某种关联关系，并且，如果我们要重新组织这个 crate 的模块树，需要一起移动它们。因此，我们使用 super，这样一来，如果这些代码被移动到了其他模块，我们只需要更新很少的代码。

## 公有结构体和枚举

我们还可以使用 pub 来设计公有的结构体和枚举，不过关于在结构体和枚举上使用 pub 还有一些额外的细节需要注意。如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。我们可以根据情况决定每个字段是否公有。

我们定义了一个公有结构体 `back_of_house:Breakfast`，其中有一个公有字段 `toast` 和私有字段 `seasonal_fruit`。这个例子模拟的情况是，在一家餐馆中，顾客可以选择随餐附赠的面包类型，但是厨师会根据季节和库存情况来决定随餐搭配的水果。餐馆可用的水果变化是很快的，所以顾客不能选择水果，甚至无法看到他们将会得到什么水果。

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn order() {
            println!("I'd like {} toast please", meal.toast);
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    meal.order();
}
```

因为 back_of_house::Breakfast 结构体的 toast 字段是公有的，所以我们可以在 eat_at_restaurant 中使用点号来随意的读写 toast 字段。注意，我们不能在 eat_at_restaurant 中使用 seasonal_fruit 字段，因为 seasonal_fruit 是私有的。

还请注意一点，因为 back_of_house::Breakfast 具有私有字段，所以这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例 (这里我们命名为 summer)。如果 Breakfast 没有这样的函数，我们将无法在 eat_at_restaurant 中创建 Breakfast 实例，因为我们不能在 eat_at_restaurant 中设置私有字段 seasonal_fruit 的值。

与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有。我们只需要在 enum 关键字前面加上 pub:

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

因为我们创建了名为 `Appetizer` 的公有枚举，所以我们可以在 eat_at_restaurant 中使用 `Soup` 和 `Salad` 成员。

如果枚举成员不是公有的，那么枚举会显得用处不大；给枚举的所有成员挨个添加 pub 是很令人恼火的，因此枚举成员默认就是公有的。结构体通常使用时，不必将它们的字段公有化，因此结构体遵循常规，内容全部是私有的，除非使用 pub 关键字。

还有一种使用 pub 的场景我们还没有涉及到，那就是我们最后要讲的模块功能：use 关键字。我们将先单独介绍 use，然后展示如何结合使用 pub 和 use。

## use 关键字

不得不编写路径来调用函数显得不便且重复。在之前的例子中，无论我们选择 add_to_waitlist 函数的绝对路径还是相对路径，每次我们想要调用 add_to_waitlist 时，都必须指定front_of_house 和 hosting。幸运的是，有一种方法可以简化这个过程。我们可以使用 use 关键字创建一个短路径，然后就可以在作用域中的任何地方使用这个更短的名字。

我们将 crate::front_of_house::hosting 模块引入了 eat_at_restaurant 函数的作用域，而我们只需要指定 hosting::add_to_waitlist 即可在 eat_at_restaurant 中调用 add_to_waitlist 函数。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

在作用域中增加 `use` 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。通过在 crate 根增加 use crate::front_of_house::hosting，现在 hosting 在作用域中就是有效的名称了，如同 hosting 模块被定义于 crate 根一样。通过 use 引入作用域的路径也会检查私有性，同其它路径一样。

注意 use 只能创建 use 所在的特定作用域内的短路径。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

示例将 eat_at_restaurant 函数移动到了一个叫 customer 的子模块，这又是一个不同于 use 语句的作用域，所以函数体不能编译。

编译器错误显示短路径不再适用于 customer 模块中：

```
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
```

注意这里还有一个警告说 use 在其作用域内不再被使用！为了修复这个问题，可以将 use 移动到 customer 模块内，或者在子模块 customer 内通过 super::hosting 引用父模块中的这个短路径。

**创建惯用的 use 路径**

你可能会比较疑惑，为什么我们是指定 use crate::front_of_house::hosting ，然后在 eat_at_restaurant 中调用 hosting::add_to_waitlist ，而不是通过指定一直到 add_to_waitlist 函数的 use 路径来得到相同的结果，

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

虽然两种方式都完成了相同的任务，但是使用 `use crate::front_of_house::hosting ` 将函数引入作用域的习惯用法。要想使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。

另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。以下示例 展示了将 HashMap 结构体引入二进制 crate 作用域的习惯用法。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

这种习惯用法背后没有什么硬性要求：它只是一种惯例，人们已经习惯了以这种方式阅读和编写 Rust 代码。

这个习惯用法有一个例外，那就是我们想使用 use 语句将两个具有相同名称的项带入作用域，因为 Rust 不允许这样做。如何将两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们。

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

如你所见，使用父模块可以区分这两个 Result 类型。如果我们是指定 use std::fmt::Result 和 use std::io::Result，我们将在同一作用域拥有了两个 Result 类型，当我们使用 Result 时，Rust 则不知道我们要用的是哪个。

**使用 as 关键字提供新的名称**

使用 use 将两个同名类型引入同一作用域这个问题还有另一个解决办法：在这个类型的路径后面，我们使用 `as` 指定一个新的本地名称或者别名。通过 as 重命名其中一个 Result 类型:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

在第二个 use 语句中，我们选择 IoResult 作为 std::io::Result 的新名称，它与从 std::fmt 引入作用域的 Result 并不冲突。

**使用 pub use 重导出名称**

使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）”：我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

在这个修改之前，外部代码需要使用路径 `restaurant::front_of_house::hosting::add_to_waitlist()` 来调用 add_to_waitlist 函数。现在这个 pub use 从根模块重导出了 hosting 模块，外部代码现在可以使用路径 `restaurant::hosting::add_to_waitlist`。

当你代码的内部结构与调用你代码的程序员所想象的结构不同时，重导出会很有用。例如，在这个餐馆的比喻中，经营餐馆的人会想到“前台”和“后台”。但顾客在光顾一家餐馆时，可能不会以这些术语来考虑餐馆的各个部分。使用 pub use，我们可以使用一种结构编写代码，却将不同的结构形式暴露出来。这样做使我们的库井井有条，也使开发这个库的程序员和调用这个库的程序员都更加方便。

**使用外部包**

我们编写了一个猜猜看游戏。那个项目使用了一个外部包，`rand`，来生成随机数。为了在项目中使用 rand，在 Cargo.toml 中加入了如下行：

文件名：Cargo.toml

```
rand = "0.8.5"
```

在 Cargo.toml 中加入 rand 依赖告诉了 Cargo 要从 crates.io 下载 rand 和其依赖，并使其可在项目代码中使用。

接着，为了将 rand 定义引入项目包的作用域，我们加入一行 use 起始的包名，它以 rand 包名开头并列出了需要引入作用域的项。我们曾将 Rng trait 引入作用域并调用了 rand::thread_rng 函数：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

crates.io 上有很多 Rust 社区成员发布的包，将其引入你自己的项目都需要一道相同的步骤：在 Cargo.toml 列出它们并通过 use 将其中定义的项引入项目包的作用域中。

注意 std 标准库对于你的包来说也是外部 crate。因为标准库随 Rust 语言一同分发，无需修改 Cargo.toml 来引入 std，不过需要通过 use 将标准库中定义的项引入项目包的作用域中来引用它们，比如我们使用的 HashMap：

```rust
use std::collections::HashMap;
```

这是一个以标准库 crate 名 std 开头的绝对路径。

**嵌套路径来消除大量的 use 行**

当需要引入很多定义于相同包或相同模块的项时，为每一项单独列出一行会占用源码很大的空间。有两行 use 语句都从 std 引入项到作用域：

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

相反，我们可以使用嵌套路径将相同的项在一行中引入作用域。这么做需要指定路径的相同部分，接着是两个冒号，接着是大括号中的各自不同的路径部分:

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

在较大的程序中，使用嵌套路径从相同包或模块中引入很多项，可以显著减少所需的独立 use 语句的数量！

我们可以在路径的任何层级使用嵌套路径，这在组合两个共享子路径的 use 语句时非常有用。例如，两个 use 语句：一个将 `std::io` 引入作用域，另一个将 `std::io::Write` 引入作用域：

```rust
use std::io;
use std::io::Write;
```

两个路径的相同部分是 `std::io`，这正是第一个路径。为了在一行 use 语句中引入这两个路径，可以在嵌套路径中使用 self:

```rust
use std::io::{self, Write};
```

这一行便将 std::io 和 std::io::Write 同时引入作用域。

**通过 glob 运算符将所有的公有定义引入作用域**

如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 `*`，glob 运算符：

```rust
use std::collections::*;
```

这个 use 语句将 `std::collections` 中定义的所有公有项引入当前作用域。使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。

glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域；

## 模块文件拆分

到目前为止，本章所有的例子都在一个文件中定义多个模块。当模块变得更大时，你可能想要将它们的定义移动到单独的文件中，从而使代码更容易阅读。

从餐厅模块的代码开始。我们会将模块提取到各自的文件中，而不是将所有模块都定义到 crate 根文件中。在这里，crate 根文件是 src/lib.rs，不过这个过程也适用于 crate 根文件是 src/main.rs 的二进制 crate。

首先将 front_of_house 模块提取到其自己的文件中 `src/front_of_house.rs` , 删除 front_of_house 模块的大括号中的代码，只留下 mod front_of_house; 声明；

文件名：src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

接下来将之前大括号内的代码放入一个名叫 `src/front_of_house.rs` 的新文件中，因为编译器找到了 crate 根中名叫 front_of_house 的模块声明，它就知道去搜寻这个文件。

文件名：src/front_of_house.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

注意你只需在模块树中的某处使用一次 `mod` 声明就可以加载这个文件。一旦编译器知道了这个文件是项目的一部分（并且通过 mod 语句的位置知道了代码在模块树中的位置），项目中的其他文件应该使用其所声明的位置的路径来引用那个文件的代码，这在“引用模块项目的路径”部分有讲到。换句话说，mod 不是 你可能会在其他编程语言中看到的 "include" 操作。

接下来我们同样将 `hosting` 模块提取到自己的文件中。这个过程会有所不同，因为 hosting 是 `front_of_house` 的子模块而不是根模块。我们将 hosting 的文件放在与模块树中它的父级模块同名的目录中，在这里是 src/front_of_house/。

为了移动 hosting，修改 src/front_of_house.rs 使之仅包含 hosting 模块的声明。

文件名：src/front_of_house.rs

```rust
pub mod hosting;
```

接着我们创建一个 src/front_of_house 目录和一个包含 hosting 模块定义的 hosting.rs 文件：

```rust
pub fn add_to_waitlist() {}
```

如果将 hosting.rs 放在 src 目录，编译器会认为 hosting 模块中的 hosting.rs 的代码声明于 crate 根，而不是声明为 front_of_house 的子模块。编译器所遵循的哪些文件对应哪些模块的代码的规则，意味着目录和文件更接近于模块树。

> **另一种文件路径**
> 目前为止我们介绍了 Rust 编译器所最常用的文件路径；不过一种更老的文件路径也仍然是支持的。
>
> 对于声明于 crate 根的 front_of_house 模块，编译器会在如下位置查找模块代码：
>
> - src/front_of_house.rs（我们所介绍的）
> - src/front_of_house/mod.rs（老风格，不过仍然支持）
>
> 对于 front_of_house 的子模块 hosting，编译器会在如下位置查找模块代码：
> - src/front_of_house/hosting.rs（我们所介绍的）
> - src/front_of_house/hosting/mod.rs（老风格，不过仍然支持）
> 如果你对同一模块同时使用这两种路径风格，会得到一个编译错误。在同一项目中的不同模块混用不同的路径风格是允许的，不过这会使他人感到疑惑。
>
> 使用 mod.rs 这一文件名的风格的主要缺点是会导致项目中出现很多 mod.rs 文件，当你在编辑器中同时打开它们时会感到疑惑。
