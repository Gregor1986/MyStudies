fn main() {
    let a = 5;
    countdown(a);
    let b = 9;
    println!("factorial {}", fact(b));
    let array = [4, 6, 7, 8, 9];
    println!("Сумма чисел в архиве: {}", sum_array(array));
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
fn sum_array(array: [i32; 5]) -> i32 {
    let mut total = 0;
    for i in array {
        total += i;
    }
    total
}