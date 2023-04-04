fn main() {
    // 分离出指数，从一个f32中分离并解析出指数
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits(); // 把f32的位模式解释为一个u32，以便执行后面操作
    let exponent_ = n_bits >> 23; // 把指数部分的8的位数据位执行右移位，覆盖尾数部分
    let exponent_ = exponent_ & 0xff; // 使用与掩码操作过滤符号位。只有最右边的8位位数据被保留下来
    let exponent = (exponent_ as i32) - 127; // 把保留下来的位数据解释成一个有符号整数，然后依据标准定义。还需要减去指数偏置量。

    println!("{} vs {}", n_bits, exponent);
}
