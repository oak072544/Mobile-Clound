use std::io;

fn main() {
    println!("Enter Input :");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error");
    let x: i32 = x.trim().parse().expect("Number");
    for a in 1..=x {
        for b in 1..=x {
            if b == 1 || b == x || b == a {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!();
    }
}
