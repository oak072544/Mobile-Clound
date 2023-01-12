use std::io;

fn main() {
    println!("Enter Input :");
    let mut x = String::new();
    let mut a=0;
    let mut b=0;
    io::stdin()
        .read_line(&mut x)
        .expect("Error");
    let x :i32 = x.trim().parse().expect("Number");
    while a < x{
        a+=1;
        for b in b..(x-a){
            print!(" ");
        }
        for b in 1..=a{
            print!("*");
        }
        for b in 1..a{
            print!("*");
        }
        println!();
    }
               
}