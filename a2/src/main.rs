fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            // Check if the found element is the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                // Continue searching in the left half for the first occurrence
                high = mid - 1;
            }
        } else if arr[mid] < target {
            // Continue searching in the right half
            low = mid + 1;
        } else {
            // Continue searching in the left half
            high = mid - 1;
        }
    }

    None // Return None if the target is not found
}

fn main() {
    let arr = [1, 2, 2, 3, 4, 4, 4, 5, 6];
    let target = 5;

    if let Some(index) = first_occurrence_index(&arr, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array.", target);
    }
}
