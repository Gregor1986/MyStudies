fn main() {
    let a = 5;
    countdown(a);
}
fn countdown(a: i32) {
    println!("{}", a);
    if a <= 0 {
        return
    } else {
        countdown(a - 1);
    }
    
}
