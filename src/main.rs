fn find_min(arr: &[i32]) -> i32 {
    let mut min = arr[0];

    for i in arr {
        if *i < min {
            min = *i;
        }
    }
    return min;
}

fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];

    for i in arr {
        if *i > max {
            max = *i;
        }
    }
    return max;
}

fn main() {
    let arr = [1, 3, 5, -1, 10, -2, 4, 7, 19, -13];
    let min = find_min(&arr);
    let max = find_max(&arr);
    println!("min: {0} | max: {1}", min, max);
}
