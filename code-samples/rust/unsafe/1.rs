use std::slice;

fn split_at_mut(values: Vec<i32>, mid: usize) -> (Vec<i32>, Vec<i32>) {
    let a: Vec<i32> = (&values[..mid]).to_vec();

    let b: Vec<i32> = (&values[mid..]).to_vec();

    println!("{a:?}");
    println!("{b:?}");

    (a, b)
}

fn split_at_mut_v2(values: &mut Vec<i32>, mid: usize) -> (&[i32], &[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}",split_at_mut(v, 3));

    let mut v = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}",split_at_mut_v2(&mut v, 3));

    v.push(7);
    println!("{v:?}");
}
