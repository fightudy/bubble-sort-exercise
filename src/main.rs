use std::cmp::PartialOrd;

fn main() {
    let raw_data = vec!(4, 6, 32, 7, 2, 7, 4, 2);
    println!("i32类型排序前：{:?}", raw_data);
    let sorted_data = bubble_sort(raw_data);
    println!("i32类型排序后：{:?}", sorted_data);

    let raw_data_t = vec!(String::from("yuiyiyu"), String::from("ddddd"),
                          String::from("asdasd"), String::from("aaddd"),
                          String::from("xcczas"), String::from("ewqeq"));
    println!("泛型排序前：{:?}", raw_data_t);
    let sorted_data_t = bubble_sort_t(raw_data_t);
    println!("泛型排序后：{:?}", sorted_data_t);
}

fn bubble_sort(mut data_list: Vec<i32>) -> Vec<i32>{
    for i in 0..data_list.len() {
        for j in i..data_list.len() {
            if data_list[i] > data_list[j] {
                data_list.swap(i, j);
            }
        }
    }
    data_list
}

fn bubble_sort_t<T: PartialOrd>(mut data_list: Vec<T>) -> Vec<T>{
    for i in 0..data_list.len() {
        for j in i..data_list.len() {
            if data_list[i] > data_list[j] {
                data_list.swap(i, j);
            }
        }
    }
    data_list
}
