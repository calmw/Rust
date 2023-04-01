#![allow(dead_code)]

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
    // 虽然File结构体是公有的，但其方法也必须显式的标记为共有的
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {}
