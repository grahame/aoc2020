use std::env;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;


fn sum_to(map: &HashMap<i32, bool>, target: i32) -> Option<(i32, i32)> {
    for i in 1..target {
        if !map.contains_key(&i) {
            continue;
        }
        let j = target - i;
        if j == i {
            continue;
        }
        if map.contains_key(&j) {
            return Some((i, j));
        }
    }
    return None;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut map = HashMap::new();
    for line in std::io::BufReader::new(file).lines() {
        let val: i32 = line.unwrap().parse::<i32>().unwrap();
        map.insert(val, true);
    }

    let (a, b) = sum_to(&map, 2020).unwrap();
    println!("a={}, b={}, a*b={}", a, b, a * b);

    for i in 1..2020 {
        if !map.contains_key(&i) {
            continue;
        }
        let j = 2020 - i;
        match sum_to(&map, j) {
            Some((a, b)) => {
                if (a != i) && (b != i) && (a != b) {
                    println!("a={}, b={}, i={}, a*b*i={}", a, b, i, a * b * i);
                }
            }
            None => {}
        }
    }
}
