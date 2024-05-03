fn main() {
    try_reference()
}

fn try_reference() {
    let s = String::from("awesome string");

    fn get_length_v1 (s: String) -> (String, usize) {
        let length = s.len();
    
        (s, length)
    }

    let (mut s1, l1) = get_length_v1(s);

    println!("string :'{s1}', length: {l1}");

    fn get_length_v2 (s: &String) -> usize {
        let length = s.len();
    
        length
    }
    
    let l2 = get_length_v2(&s1);

    println!("string :'{s1}', length: {l2}");

    fn mut_str (s: &mut String) -> &mut String {
        s.push_str(", mutant");
        s
    }

    let s3 = mut_str(&mut s1);

    println!("s3 :'{s3}'");
    println!("s1 :'{s1}'");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn try_ownership() {
    fn take_and_return(some_string: String) -> String {
        some_string
    }

    let s = String::from("some string");

    println!("{}", s);
    
    let n = 0;

    let s1 = String::from("string");

    // let s2 = s1; -> let s2 = s1.clone();
    let s2 = s1.clone();

    // let s3 = take_and_return(s1); -> let s3 = take_and_return(s1.clone());
    let s3 = take_and_return(s1.clone());

    println!("s: {s}");
    println!("n: {n}");
    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");
}
