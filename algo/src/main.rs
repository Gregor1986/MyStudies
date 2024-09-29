fn main() {
    let mut a = vec![];
    let len_vec = 100;
    let mut i = 1;
    while i <= len_vec {
        a.push(i);
        i += 1;
    }
    println!("{:?}", a);
}
