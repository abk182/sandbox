fn main() {
    let mut vector = vec![10, 7, 7, 84, 25, 10, 7];

    println!("{}", get_median(&mut vector));
    println!("{}", get_mode(&vector));
}

fn get_median(vector: &mut Vec<i32>) -> i32 {
    vector.sort();
    vector[vector.len() / 2]
}

fn get_mode(vector: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut max: i32 = 0;
    let mut output: i32 = 0;

    for i in vector {
        let count = map.entry(i).or_insert(0);
        *count += &*count + 1;

        if *count > max {
            max = *count;
            output = *i;
        }
    }

    output
}
