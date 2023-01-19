fn main() {
    //A
    let number =100;

    //step1
    let mut sum=(number/2)*(number+1);

    //B
    println!("The sum of the first {} natural numbers is: {}", number, sum);
}
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_sum() {
       // Set the expected output 
       let number =100;
       let mut sum=(number/2)*(number+1);
       assert_eq!(sum,5050)
   }
}