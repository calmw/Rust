struct Hostname(String); // Hostname是新类型

fn connect(host: Hostname) {// 使用类型系统来防范无效的用法。
    println!("connected to {}", host.0) // 使用一个数字索引来访问其底层数据
}

fn main() {}