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
