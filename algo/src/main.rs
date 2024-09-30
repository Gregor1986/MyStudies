fn main() {
    let vec_a = create_vec(10000);
    let item = 5;
    position_in_vec(vec_a, item);
    
}
// Создание вектора
fn create_vec(len_vec: i32) -> Vec<i32> {
    let mut a = vec![];
    let mut i = 1;
    while i <= len_vec {
        a.push(i);
        i += 1;
    }
    a
}
// Функция бинарного поиска
fn position_in_vec(vec: Vec<i32>, item: i32) {
    let mut low = 0;
    let mut high = vec.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        let guess = vec[mid];
        if guess == item {
            println!("Позиция элемента в векторе {}", mid);
            break;
        } else if guess > item {
            high = mid - 1
        } else {
            low = mid + 1
        }
    }
}