use std::io;

fn main() {
    println!("Enter Input :");
    let mut x = String::new();
    let mut a=0;
    let mut b=0;
    let mut c=0;
    io::stdin()
        .read_line(&mut x)
        .expect("Error");
    let x :i32 = x.trim().parse().expect("Number");
    while a < x{
        a+=1;
        for b in b..(a-1){
            print!(" ");
        }
        for b in b..=(x-a){
            print!("*");
        }
        c = x-a;
        c-=1;
        for c in 1..=(c+1){
            print!("*");
        }
        println!();
    }
               
}