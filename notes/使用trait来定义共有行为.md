#### 介绍

- trait让编译器（和其他人）知道，有多个类型试图执行同一个任务。例如使用了#[derive(Debug)]注解的所有类型，都能够通过println!
  宏和其他相关的宏来输出信息到控制台。允许多个类型去实现一个trait，是的代码能够被重用，而且能让编译器去执行他的零成本抽象的"
  魔法"
- trait是用来表示某些不同类型所具有的共同行为的，使用 impl Trait for Type这个语法来实现

#### 实现Display trait的例子

- 要为一个结构体实现Display，此结构体中的字段也需要实现Display

``` rust
#![allow(dead_code)]              // <1>

use std::fmt;
// <2>
use std::fmt::{Display};          // <3>

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),      // 在这里，我们悄悄利用了write!来做复杂的工作。字符串本身已经实现了Display
            FileState::Closed => write!(f, "CLOSED"),  // <4>
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>",
               self.name, self.state)              // 我们可以依赖于FileState类型的Display实现
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);           // <6>
    println!("{}", f6);             // <7>
}

```