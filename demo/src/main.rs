fn main() {
    let n = 123456;
    let description = match true { // 这里的true可以是其他表达式（比如带有n的函数）
        true => "even",
        false => "odd",
    };
    println!("{} is {}", n, description)
}
