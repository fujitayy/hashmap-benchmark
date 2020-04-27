use std::collections::HashMap;
use std::hash::Hash;
use std::time;

fn main() {
    simple_hash();
    complex_hash();
}

fn simple_hash() {
    let start = time::Instant::now();

    let mut m: HashMap<i64, i64> = HashMap::new();
    for i in 0..10000 {
        m.insert(i, i);
    }
    let mut ans = 0;
    for k in m.keys() {
        ans += m[k];
    }

    let end = start.elapsed();

    println!("[simple_hash key:int] answer={}, {} sec", ans, end.as_secs_f64());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    k1: Key2,
    k2: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key2 {
    k1: i64,
    k2: i64,
}

fn gen_keys() -> Vec<Key> {
    let chars = ["あ", "い", "う", "え", "お"];
    let mut keys = Vec::new();
    for _ in 0..10000 {
        keys.push(Key {
            k1: Key2 {
                k1: rand::random::<i64>() % 100,
                k2: rand::random::<i64>() % 100,
            },
            k2: itertools::join(
                &[
                    chars[rand::random::<usize>() % 5],
                    chars[rand::random::<usize>() % 5],
                    chars[rand::random::<usize>() % 5],
                ],
                "",
            ),
        });
    }
    return keys;
}

fn complex_hash() {
    let keys = gen_keys();
    let start = time::Instant::now();

    let mut m: HashMap<Key, Key> = keys.iter().fold(HashMap::new(), |mut m, k| {
        m.insert(k.clone(), k.clone());
        m
    });
    let ans = m.keys().into_iter().fold(0, |mut acc, k| {
        acc += k.k1.k2;
        acc
    });

    let end = start.elapsed();

    println!("[complex_hash key:int/int/stringな構造体] answer={}, {} sec", ans, end.as_secs_f64());
}
