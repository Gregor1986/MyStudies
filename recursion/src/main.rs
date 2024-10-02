fn main() {
    let a = 5;
    countdown(a);
    let b = 9;
    println!("factorial {}", fact(b));
    let array = [4, 6, 7, 8, 9, 56];
    println!("Сумма чисел в архиве: {}", sum_array(array));
    println!("{}", sum_recursion(array));
    println!("{}", count_element(array));
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
fn sum_array(array: [i32; 6]) -> i32 {
    let mut total = 0;
    for i in array {
        total += i;
    }
    total
}
fn sum_recursion(array: [i32; 6]) -> i32 {
    let mut key = 0;
    for i in 0..array.len() {
        key = array[i];
    }
    key
}
fn count_element(array: [i32; 6]) -> i32 {
    let mut count = 0;
    // let mut key = 0;
    for i in 0..array.len() {
        if count > i {
            count as i32;
        } else {
            count += 1;
        }
    }
    count as i32
}