#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
struct AnotherPoint {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let another_p2 = AnotherPoint { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
    println!("{}", p1 == p2);
    // => The runtime will raise an error due to p1 and another_p2 are different data types
    // println!("{}", p1 == another_p2);
    println!("{}", p1 == p3);
}