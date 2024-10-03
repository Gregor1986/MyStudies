use rand::{Rng, thread_rng};
fn main() {
    let mut rng = thread_rng();
    // let x: u32 = rng.gen();
    // println!("{}", x);
    // let n: u32 = rng.gen_range(0..100);
    // println!("{}", n);
    let mut arr = [0u8; 20];
    thread_rng().fill(&mut arr[..]);
    println!("array: {:?}", arr);
    println!("vector: {:?}", generate_vector(10));

}
fn generate_vector(lenght_vec: usize) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![];
    let mut i: usize = 0;
    let mut rng = thread_rng();
    while i != lenght_vec {
        let number = rng.gen_range(0..100);
        vector.insert(i, number);
        i += 1;
    }
    vector
}