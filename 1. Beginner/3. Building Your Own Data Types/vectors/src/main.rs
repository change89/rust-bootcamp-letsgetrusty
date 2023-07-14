fn main() {
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let v2 = vec![1, 2, 3];

    let s = &v[0];
    // We must have & here.
    // The vector v is borrowed as immutable by the reference &v[0],
    // and Rust prevents moving values out of borrowed content to maintain memory safety.

    let s = v.get(0);

    if let Some(e) = s {
        println!("{e}");
    }

    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    let mut v3 = vec![];

    for s in v.into_iter() {
        v3.push(s);
    }

    // let i = v.get(0);
}
