#[allow(arithmetic_overflow)] // arithmetic 算术。告诉编译器允许算数溢出. 这样的代码可以编译通过，但运行会出现panic
fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b; // 如果没有这个类型声明，rust不会假设你想要创建一种不可能的情况
    println!("200 + 200 = {}", c);
}
