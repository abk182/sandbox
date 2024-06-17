fn main() {
    let v1 = vec![1, 2, 3];
    for val in &v1 {
        println!("Got: {val}");
    }

    let v1_iter = v1.iter();
    println!("{:?}", v1);
    println!("{:?}", v1_iter);

    for val in v1_iter {
        println!("Got: {val}");
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
    println!("{:?}", v2);
    fn custom_filter(v: Vec<i32>, t: i32) -> Vec<i32> {
        v.into_iter().filter(|s| s > &t).collect()
    }
    let v3: Vec<_> = custom_filter(v2, 2);
    println!("{:?}", v3);

}
