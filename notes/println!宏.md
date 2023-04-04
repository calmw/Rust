#### 介绍

    println!("{:032b}", frankentype)。 {:032b},在这里032的意思时"占用32，位数不足的在其左侧补零"，在其右边的b会调用std::fmt::Binary trait。在这里如果使用的时默认语法{},则会调用std::fmt::Display trait。如果使用问好语法{:?}则会调用std::fmt::Debug trait
