#![allow(dead_code)]
//! 表示当前语言项的文档，即编译器刚刚进入的哪个模块

/// 用于注解其后紧跟着的语言项1
/// 用于注解其后紧跟着的语言项2


use std::fmt;
use std::fmt::{Display};

#[derive(Debug, PartialEq)]
// 如果将整个枚举标记为公有的，则其变体也为公有的
pub enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    // 当第三方代码使用use来导入这个包时，File.data依然保持为私有的
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    /// New files are assumed to be empty
    /// ## new 方法注释
    /// ```
    /// let f = File::new("f1.txt")
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {}
