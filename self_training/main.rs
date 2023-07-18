/*
fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        //! Due to quicksort return () - an empty tuple
        //! return;
        //! => The return statement is used to immediately exit the function without any explicit return value
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    quicksort(&mut arr);
    println!("{:?}", arr); // Output: [1, 2, 5, 5, 6, 9]
}
*/

// Adding debug code
fn quicksort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    
    //! Due to quicksort return () - an empty tuple
    //! return;
    //! => The return statement is used to immediately exit the function without any explicit return value
    if arr.len() <= 1 {return;}
    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}


fn partition<T: Ord + std::fmt::Debug>(arr: &mut [T]) -> usize {
    println!("before sorting: {:?}", arr);
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    println!("after sorting: {:?}", arr);
    
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    quicksort(&mut arr);
    println!("{:?}", arr); // Output: [1, 2, 5, 5, 6, 9]
}