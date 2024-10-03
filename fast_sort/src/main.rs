use rand::{Rng, thread_rng};
fn main() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen();
    println!("{}", x);
    let n: u32 = rng.gen_range(0..100);
    println!("{}", n);
    let mut arr = [0u8; 20];
    thread_rng().fill(&mut arr[..]);
    println!("array: {:?}", arr);
    println!("vector: {:?}", generate_vector(10));
    let sort_vector = quick_sort(generate_vector(100));
    println!("{:?}", sort_vector);
}
fn generate_vector(lenght_vec: usize) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![];
    let mut i: usize = 0;
    let mut rng = thread_rng();
    while i != lenght_vec {
        let number = rng.gen_range(0..1000);
        vector.insert(i, number);
        i += 1;
    }
    vector
}
fn quick_sort<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr;
    }
    // присваиваем 0 элемент вектора, при этом удаляем его из вектора
    let pivot = arr.remove(0);
    let mut left = vec![];
    let mut right = vec![];
    for item in arr {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }
    let mut sorted_left = quick_sort(left);
    let mut sorted_right = quick_sort(right);
    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);
    sorted_left
}