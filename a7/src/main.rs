fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).cloned()
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6];
    let k = 5;
    if let Some(kth_smallest) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Array is empty or k is out of bounds.");
    }
}
