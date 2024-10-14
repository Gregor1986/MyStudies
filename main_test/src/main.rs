fn main() {
    let a = [5, 6, 7, 3, 2, 1, 9];
    println!("{}", average(&a));
    let n: usize = 15;
    let xs = random(n);
    println!("{:?}", xs);
}
// Среднее значение сумма чисел в массиве
fn average (array: &[i32]) -> f64 {
    array.iter().fold(0, |x, y| x + y) as f64 / array.len() as f64
}
// Псевдо генератор вектора
fn random(n: usize) -> Vec<u32> {
    let mut r = 9;
    std::iter::repeat_with(move || {
        r ^= r << 3;
        r ^= r >> 17;
        r ^= r << 5;
        r
    }).take(n).collect()
}