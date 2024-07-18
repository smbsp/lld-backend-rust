// Note: While this implementation demonstrates the use of threads for concurrent sorting, it's important 
// to consider the overhead of thread creation and synchronization. For small arrays or a large number 
// of threads, the overhead may outweigh the benefits of parallelism.

use std::thread;

fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    println!("Thread {:?}: Sorting {:?}", thread::current().id(), nums);

    if nums.len() <= 1 {
        return nums;
    }

    let mid = nums.len() / 2;
    let (left, right) = nums.split_at_mut(mid);

    // Clone the slices into new vectors
    let left_vec = left.to_vec();
    let right_vec = right.to_vec();

    // Sort the left and right halves concurrently using threads
    let left_thread = thread::spawn(move || merge_sort(left_vec));
    let right_thread = thread::spawn(move || merge_sort(right_vec));

    let left_sorted = left_thread.join().unwrap();
    let right_sorted = right_thread.join().unwrap();

    // Merge the sorted halves
    let sorted = merge(left_sorted, right_sorted);
    println!("Thread {:?}: Merged {:?}", thread::current().id(), sorted);
    sorted
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            merged.push(left[left_index]);
            left_index += 1;
        } else {
            merged.push(right[right_index]);
            right_index += 1;
        }
    }

    while left_index < left.len() {
        merged.push(left[left_index]);
        left_index += 1;
    }

    while right_index < right.len() {
        merged.push(right[right_index]);
        right_index += 1;
    }

    merged
}

fn main() {
    let nums = vec![38, 27, 43, 3, 9, 82, 10];
    let sorted = merge_sort(nums);
    println!("Final sorted array: {:?}", sorted);
}
