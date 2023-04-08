fn main() {
    let a = Box::new(1); // 在堆上分配内存， a 为指向整数的引用，
    let b = Box::new(1);
    let c = Box::new(1);
    let result1 = *a + *b + *c;
    drop(a); // drop() 用于在对象所在的作用域结束之前删除对象，实现了Drop的类型，就会有一个drop方法，但是在用户代码中显式调用它是不合法的。而使用std::mem::drop则可以绕过这个规则
    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("{} {}", result1, result2)
}