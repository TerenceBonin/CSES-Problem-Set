use std::io::{self, BufRead};
use std::cmp;

fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let char_list : String = iterator.next().unwrap().unwrap().trim().parse().expect("Input not an integer");
    let mut old_char : char = 'm';
    let mut aggregator : i64 = 1;
    let mut longest : i64 = 0;
    for character in char_list.chars(){
        if old_char == character{
            aggregator += 1;
        }
        else{
            longest = cmp::max(aggregator,longest);
            aggregator = 1;
        }
        old_char = character;
    }
    longest = cmp::max(aggregator,longest);
    println!("{}", longest)

}