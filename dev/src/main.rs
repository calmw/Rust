use std::fs::File;
// BufReader负责提供I/O（buffered I/O），这样可以减少对操作系统的调用，也就是说减少对硬盘的读取次数。
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep=lite") // 逐行构建命令行参数解析器。每个参数对应一个Arg。
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {    // line是String类型，但是re.find()接收类型为&str的参数，
            Some(_) => println!("{}", line),    // 代表re.find()方法查找成功。Some中的_是通配符，匹配所有值
            None => (),    // 空的占位符的值
        }
    }
}

// 执行 cargo run Cisco -  , 然后输入 Hi Cisco
