use rand::{Rng, thread_rng};
fn main() {
    let mut rng = thread_rng();
    // let x: u32 = rng.gen();
    // println!("{}", x);
    // let n: u32 = rng.gen_range(0..100);
    // println!("{}", n);
    // let mut arr = [0u8; 20];
    // thread_rng().fill(&mut arr[..]);
    // println!("{:?}", arr);
    // println!("{:?}", generate_vector(10));
    let mut vector: Vec<i32> = vec![];
    // let number: i32 = rng.gen_range(0..100);
    println!("{}", vector.len());
    let mut i = 0;
    while i != 10 {
        let number: i32 = rng.gen_range(0..100);
        vector.insert(i, number);
        i += 1;
    }
    // // vector.push(number);
    println!("{:?} lenght {}", vector, vector.len());
}