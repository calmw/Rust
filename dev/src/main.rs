#[derive(Debug)]
struct Demo {
    a: i32,
}

fn send(to: CubeSat, msg: Message) {
    to.mailbox.messages.push(msg)
}

fn send(to: &mut CubeSat, msg: Message) {
    to.mailbox.messages.push(msg)
}

fn use_value(_val: Demo) {}

fn main() {
    let a = Demo { a: 3 };
    use_value(a);
    println!("{:?}", a)
}
// 编译不通过
