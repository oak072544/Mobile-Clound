use std::io;

fn main() {
    println!("Enter Number :");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Error");
    let x: i32 = x.trim().parse().expect("Number");
    let mut a=x;
    let mut b=true; 
    print!("{} = ", x);
    for c in 2..{
        while a%c==0{
            if b{
                b=false;
            }
            else{
                print!("*");
            }
            print!("{}",c);
            a/=c;
        }
        if a==1{
            break;
        }
    }
}