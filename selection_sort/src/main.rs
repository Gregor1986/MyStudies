fn main() {
    let unsort_array = [43, 64, 13, 96, 45, 87, 23];
    // println!("{}", min_element(unsort_array));
    println!("{:?}", selection_sort(unsort_array));
}
fn min_element(array: [i32; 7]) -> i32 {
    let mut min = array[0];
    let mut min_index = 0;
    for i in 0..array.len() {
        if min > array[i] {
            min = array[i];
            min_index = i;
        }
    }
    array[min_index]
}
fn selection_sort(mut array: [i32; 7]) -> [i32; 7] {
    // let new_array: [i32; 7];
    for i in 0..array.len() {
        let min = min_element(array);
        array[i] = min;
    }
    array
}