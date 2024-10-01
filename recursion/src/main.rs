fn main() {
    let a = 5;
    countdown(a);
    let b = 9;
    println!("factorial {}", fact(b));
}
fn countdown(a: i32) {
    println!("{}", a);
    if a <= 0 {
        return
    } else {
        countdown(a - 1);
    }
}
fn fact(b: i32) -> i32 {
    if b == 1 {
        b
    } else {
        b * fact(b - 1)
    }
}