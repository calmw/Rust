fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {  // 使用unsafe关键字并不表示此段代码具有内在的危险性，比如它不允许你绕过rust的借用检查器。在unsafe块允许的各种功能中，某些功能要比其他一些功能更加难以验证。就来说std::mem::transmute()函数就是rust语言中最不安全的功能之一，它已经没有任何类型的安全性可言。因此你要在使用它之前，请先研究有没有其他替代方案。
        std::mem::transmute(a)  // std::mem::transmute()函数要求rust在不影响任何底层位数据的情况下，把一个f32直接解释成一个u32。
    };

    println!("{}", frankentype);              // <2>
    println!("{:032b}", frankentype); // {:032b},在这里032的意思时"占用32，位数不足的在其左侧补零"，在其右边的b会调用std::fmt::Binary trait。在这里如果使用的时默认语法{},则会调用std::fmt::Display trait。如果使用问好语法{:?}则会调用std::fmt::Debug trait

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };
    println!("{}", b);
    assert_eq!(a, b);                        // <4>
}