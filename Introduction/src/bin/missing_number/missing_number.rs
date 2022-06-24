use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let n: i32 = iterator.next().unwrap().unwrap().trim().parse().expect("Input not an integer");
    let list_n: Vec<_> = iterator.next().unwrap().unwrap().split(" ")
                            .map(|x| x.parse().expect("Input not an integer"))
                            .collect();

    let range_n : Vec<_> = (0..n+1).collect();

    let sum_range_n : i32 = range_n.iter().sum();
    let sum_list_n : i32 = list_n.iter().sum();

    println!("{}", sum_range_n-sum_list_n);
}