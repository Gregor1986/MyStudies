fn main() {
    let unsort_massive = [43, 64, 85, 96, 45, 87, 23];
    let sort_massive = unsort_massive
        .map(|x| x + 1);
    println!("{:?}", sort_massive);
    // let mut sort_massive[i32; unsort_massive.len()] = [];
    // let mut smallest = unsort_massive[0];
    // let mut smallest_index = 0;
    // for i in 1..unsort_massive.len() {
    //     if unsort_massive[i] < smallest {
    //         smallest = unsort_massive[i];
    //         smallest_index = i;
    //     }
    //     println!("{}", );
    // }
}
