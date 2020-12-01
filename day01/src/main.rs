use std::env;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut map = HashMap::new();
    for line in std::io::BufReader::new(file).lines() {
        let val: i32 = line.unwrap().parse::<i32>().unwrap();
        map.insert(val, true);
    }
    for i in 1..2020 {
        if !map.contains_key(&i) {
            continue;
        }
        let j = 2020 - i;
        if map.contains_key(&j) {
            println!("{}, {}, {}", i, j, i * j);
        }
    }
}
