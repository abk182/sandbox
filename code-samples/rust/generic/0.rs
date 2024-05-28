#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<f32, f32> {
    fn mix(&mut self) -> Point<f32, f32> {
        let prev_x = self.x;
        self.x = self.y;
        self.y = prev_x;
        Point {
            x: self.y,
            y: self.x
        }
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other_point: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other_point.y
        }
    }
}

fn main() {
    let mut p0 = Point { x: 5.0, y: 10.0 };
    p0.mix();
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 'a', y: 'b' };
    let p3 = p2.mixup(p1);
    println!(
        "p0.x = {}, distance_from_origin = {}, p3 = {:?}",
        p0.x(),
        p0.distance_from_origin(),
        p3
    );

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['a', 'b', 'z'];

    let result = largest(&number_list);
    println!("The largest char is {}", result);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
