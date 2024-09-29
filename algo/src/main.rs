fn main() {
    println!("{:?}", create_vec(1000));
}
fn create_vec(len_vec: i32) -> Vec<i32> {
    let mut a = vec![];
    let mut i = 1;
    while i <= len_vec {
        a.push(i);
        i += 1;
    }
    a
}