use std::collections::HashMap;

fn main() {
    let mut hash_map = HashMap::new();
    let string = "abcdacdfgh";

    for chr in string.chars() {
        let count = hash_map.entry(chr).or_insert(0);
        *count += 1;
    }

    // for chr in string.chars() {
    //     // ! Modify the counter of every chars
    //     hash_map.entry(chr)
    //         .and_modify(|count| *count += 1)
    //         .or_insert(0);
    // }

    println!("{:?}", hash_map)
}