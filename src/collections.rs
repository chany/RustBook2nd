use std::collections::HashMap;

fn main() {
    let v = vec![1,2,3,4,4,4,4,4,5,5,5,6,7,8,9];

    println!("mean: {}", mean(&v));
    println!("median: {}", median(&v));
    let m = mode(&v);
    println!("mode: {} x {}", m.0, m.1);

    let s = String::from("first apple world");
    println!("from \"{}\" to \"{}\"", s, piglatin(&s));
}

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum = 0;

    for num in v {
        sum += num;
    }

    sum as f32 / v.len() as f32
}

fn median(v: &Vec<i32>) -> i32 {
    let mut mv = v.clone();
    mv.sort();
    mv[v.len()/2]
}

fn mode(v:&Vec<i32>) -> (i32, i32) {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode_tuple = (0, 0);
    for (k, v) in map {
        if v > mode_tuple.1 {
            mode_tuple = (*k, v);
        }
    }
    
    mode_tuple
}

fn piglatin(s: &str) -> String {
    let mut result = String::new();

    for word in s.split_whitespace() {
        let c = word.chars().next().unwrap();
        match c {
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => result.push_str(&format!("{}-hay ", word)),
            _ => result.push_str(&format!("{}-{}ay ", &word[1..], &c)),
        }
    }
    result
}