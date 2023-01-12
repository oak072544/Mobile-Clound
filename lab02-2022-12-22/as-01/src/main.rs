use std::io;

fn main() {
    println!("Enter Input :");
    print!("");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Error");
    let y :i32 = x.trim().parse().expect("Number");
    for y in 1..=y{
        for y in 1..=y{
            print!("*");
        }
        println!();
    }
}