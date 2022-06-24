use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    let mut x: i64 = buffer.trim().parse().expect("Input not an integer");
    println!("{}",x);
    while x != 1 {
        match x%2{
            0=>{x /= 2;
                println!("{}",x)
            }
            1=>{x = 3*x+1;
                println!("{}",x)
            }
            _=>println!("alrightlol")
        }
    }
}
