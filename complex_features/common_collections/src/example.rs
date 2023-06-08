use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    // Given a list of integers,
    // use a vector and return the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.

    let list = vec![4,2,5,7,8,3,3,5,7,8,8,5,3,2,4,7,8,7,4];

    let list = quicksort(list);
    let map = count_list(&list);
    println!("sorted list: {:?}", list);
    println!("count map: {:?}", map);

    let median: f32 = match list.len() % 2 {
        0 => (list[list.len() / 2 - 1] + list[list.len() / 2]) as f32 / 2f32,
        1 => list[list.len() / 2] as f32,
        _ => 0.0
    };
    println!("median is {}", median);

    let mut modes : Vec<i32> = Vec::new();
    let mut mode_count = 0;
    for (i, c) in &map {
        match c.cmp(&mode_count) {
            Ordering::Less => (),
            Ordering::Greater => {
                modes = Vec::new();
                modes.push(*i);
                mode_count = *c
            },
            Ordering::Equal => {
                modes.push(*i);
            }
        }
    }
    println!("modes: {:?}, mode_count: {}", modes, mode_count);
}


fn quicksort(v: Vec<i32>) -> Vec<i32> {
    match v.get(0).copied() {
        Some(x) => {
            let mut y = Vec::new();
            let mut z = Vec::new();
            for k in &v[1..] {
                if *k <= x {
                    y.push(*k);
                } else {
                    z.push(*k);
                }
            }
            let mut y = quicksort(y);
            y.push(x);
            let mut z = quicksort(z);
            y.append(&mut z);
            y
        },
        None => v
    }
}

fn count_list(v: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for l in v {
        let count = map.entry(*l).or_insert(0);
        *count += 1;
    }
    map
}