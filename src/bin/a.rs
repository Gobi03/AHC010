#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const SIDE: usize = 30;

struct Input {
    t: Vec<Vec<usize>>,
}
impl Input {
    fn new(t: Vec<Vec<char>>) -> Self {
        Self {
            t: t.iter()
                .map(|cs| cs.iter().map(|&c| c as usize - 48).collect())
                .collect(),
        }
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();
    let mut _rng = thread_rng();

    input! {
        t: [Chars; SIDE],
    }

    let input = Input::new(t);

    for _ in 0..900 {
        print!("0")
    }

    println!("");

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
