fn main() {
    'out: for i in 0.. {
        for j in 0.. {
            for k in 0.. {
                if i + j + k > 1000 {
                    println!("{}", i + j + k);
                    break 'out;
                }
            }
        }
    }
}
