fn find_min(arr: &[i32]) -> Option<i32> {
    arr.iter().min().copied()
}

fn find_max(arr: &[i32]) -> Option<i32> {
    arr.iter().max().copied()
}

fn binsearch(x: i32, v: &[i32]) -> Option<usize> {
    let mut left = 0;
    let mut right = v.len();

    while left <= right {
        let mid = (left + right) / 2;
        if v[mid] == x {
            return Some(mid);
        } else if v[mid] < x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    let arr = [1, 3, 5, -1, 10, -2, 4, 7, 19, -13];
    let index = 4;

    match binsearch(index, &arr) {
        Some(index) => println!("Result: {0}", index),
        None => println!("Not found"),
    }

    match (find_min(&arr), find_max(&arr)) {
        (Some(min), Some(max)) => println!("min: {0} | max: {1}", min, max),
        _ => println!("Array is empty."),
    }
}
