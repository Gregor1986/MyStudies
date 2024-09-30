fn main() {
    let unsort_array = [43, 64, 13, 96, 45, 87, 23];
    // найти самый маленький элемент в массиве
    let mut min = unsort_array[0];
    let mut min_index = 0;
    for i in 0..unsort_array.len() {
        if min > unsort_array[i] {
            min = unsort_array[i];
            min_index = i;
        }
    }
    println!("{}, {}", min_index, unsort_array[min_index]);
    
    // for j  in 1..unsort_massive.len() {
    //     // Элемент массива
    //     let key = unsort_massive[j];
    //     // Ключ предыдущего элемента массива
    //     let mut i = j as i32 - 1;
    //     // Пока ключ передыдущего элемента больше 0 и предыдущий элемент больше 
    //     // текущего элемента. Проходим с самого начал сравнивая
    //     while (i >= 0) && (unsort_massive[i as usize] > key) {
    //         // Меняем местами текущий элемент с предыдущим
    //         unsort_massive[i as usize + 1] = unsort_massive[i as usize];
    //         i = i - 1;  
    //     }
    //     // Перезаписываем в массив
    //     unsort_massive[(i + 1) as usize] = key;
    // }
    // println!("{:?}", unsort_massive);
    // for i in 0..unsort_massive.len() {
    //     let key = unsort_massive[i];
    //     let mut j = 0;
    //     if unsort_massive[j] <= unsort_massive[i] {
    //         unsort_massive[(j + 1) as usize] = key;
    //         j += 1;
    //     }   
    //     println!("{:?}", unsort_massive);
    // }   

}