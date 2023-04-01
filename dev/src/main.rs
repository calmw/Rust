#![allow(unused_variables)] // 在构思过程中放宽编译器警告

fn main() {
    print!("a");
}

#[allow(dead_code)] // 放宽一个未使用函数的编译器警告
fn read() {}

fn dead_end() -> ! {
    panic!("you have reached a dead code"); // panic！宏会导致程序崩溃，这意味着此函数保证用不返回。
}

fn forever() -> ! {
    loop { // loop将永远不会结束循环。这阻止了此函数返回。如果包含了break，编译就会出错，因为返回了（），需要修改返回值类型
        // ...
        break;
    }
}