fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?"; // 要表示一个多行字符串，并不需要特殊的语法。反斜线（\）字符会使编译器忽略掉紧跟着的换行符

    for line in quote.lines() { // lines()返回一个引用的迭代器，每次迭代都返回此文本中的一行。Rust会按照每种操作系统的约定来表示相应的换行符。quote.lines()表示按行迭代，并且是以跨平台的方式来实现的。
        if line.contains(search_term) { // line.contains() 执行文本的查找
            println!("{}", line);
        }
    }

    for (i, line) in quote.lines().enumerate() { // lines()返回一个引用的迭代器，可以和enumerate形成链式调用。
        if line.contains(search_term) {
            println!("{}: {}", i + 1, line);
        }
    }
}

