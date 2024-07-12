fn main() {
    let color: Option<&str> = Some("red");

    match color {
        Some(c) => {
            println!("{}", c);
            c
        }
        None => "unknown",
    };

    if let Some(c) = color {
        println!("{}", c);
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let Some(s): Option<&str> = Some("aaaa") else {
        panic!("AAAA")
    };
    println!("{}", s);

    fn some_value((Some(_) | None): Option<&str>) {}

    let tuple = (1, 2, 3);
    // let (x, y) = tuple; break;
    let (x, y, ..) = tuple;
    let array = [1, 2, 3];

    println!("{}", tuple.0);
    println!("{}", array[0]);

    some_value(Some("some value"));
    some_value(None);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point {x, y: y2 @ 0..=7 } =>  println!("On the x axis at y in range, y2: {y2}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    match p {
        Point { x, .. } => println!("x is {x}"),
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

