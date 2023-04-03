#### 介绍

| 克隆（std::clone::Clone）                                          | 复制（std::marker::Copy）                                            |
|----------------------------------------------------------------|------------------------------------------------------------------|
| clone是显式的发生作用的，实现了clone的类型，都会有一个.clone方法，该方法可以执行创建一个新类型所需的各种操作 | 对于CPU来说复制是廉价的、开销很低的。正因为廉价这个事实，rust总是会复制基本类型的值，而不是以别的方式去考虑他的所有权移动 |
| 可能是速度慢且昂贵的                                                     | 总是快速且廉价的                                                         |
| 永远不会隐式的发生，总是需要显式的调用clone方法                                     | 总是隐式的发生                                                          |
| 在具体行为上可能会有差别，软件包的作者会为包中的类型定义出克隆的具体含义                           | 行为上总是相同的，总是按位复制原本的值                                              |

#### 实现Copy

- 1 通过derive来实现copy

```rust
#[derive(Copy, Clone, Debug)] // 告诉编译器自动为每一个trait添加一个实现
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}
```

- 2 手动实现copy

``` rust
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

// 在手动实现下面的Copy之前，需要先实现Clone，因为实现Copy的前提条件的实现Clone
impl Clone for CubeSat {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Copy for CubeSat {}
```