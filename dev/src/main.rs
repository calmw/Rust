use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,  // Mhz
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
        GroundStation {
            radio_freq: 87.65
        }
    ));

    println!("base: {:?}", base);

    // 引入一个新的作用域，在此作用域中对base执行了可变借用
    {                                            // <1>
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base); // base: RefCell { value: <borrowed> } ，value: <borrowed>表示base被别处的代码进行了可变借用，不能再进行常规的访问。
    println!("base_3: {:?}", base_3);
}
