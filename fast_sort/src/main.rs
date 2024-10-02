use rand::{Rng, thread_rng};
fn main() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen();
    println!("{}", x);
    let n: u32 = rng.gen_range(0..100);
    println!("{}", n);
    let mut arr = [0u8; 20];
    thread_rng().fill(&mut arr[..]);
    println!("{:?}", arr);
}
