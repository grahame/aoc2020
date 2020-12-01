use std::env;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;


fn sum_to(map: &HashMap<i32, bool>, target: i32) -> (i32, i32) {
    for i in 1..target {
        if !map.contains_key(&i) {
            continue;
        }
        let j = target - i;
        if map.contains_key(&j) {
            return (i, j);
        }
    }
    return (-1, -1);
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

    let (a, b) = sum_to(&map, 2020);
    println!("a={}, b={}, a*b={}", a, b, a * b);
}
