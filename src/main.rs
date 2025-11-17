fn find_min(arr: &[i32]) -> Option<i32> {
    arr.iter().min().copied()
}

fn find_max(arr: &[i32]) -> Option<i32> {
    arr.iter().max().copied()
}

fn main() {
    let arr = [1, 3, 5, -1, 10, -2, 4, 7, 19, -13];

    match (find_min(&arr), find_max(&arr)) {
        (Some(min), Some(max)) => println!("min: {0} | max: {1}", min, max),
        _ => println!("Array is empty."),
    }
}
