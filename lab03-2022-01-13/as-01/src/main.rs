
fn main() {
    
    //A
    let string = "this cat this bat this rat";

    //step1 word
    let mut split : Vec<&str> = string.split(" ").collect();
    
    //step2 uniqe
    let uniqed = uniqe(split);
    println!("{:?}", uniqed);


    //step3 count
    let mut length = uniqed.len();

    //B
    println!("{}", length);
}

fn uniqe(split: Vec<&str>) -> Vec<&str> {
    let mut uniqe = Vec::new();
    for i in &split {
        if !uniqe.contains(i) {
            uniqe.push(i);
        }
    }
    uniqe
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_word() {
       // Set the expected output 
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       assert_eq!(split,["this","cat","this","bat","this","rat"])
   }
   #[test]
   fn test_unique() {
       // Set the expected output 
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       let uniqed = uniqe(split);
       assert_eq!(uniqed,["this","cat","bat","rat"])
   }
   #[test]
   fn test_count() {
       // Set the expected output 
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       let uniqed = uniqe(split);
       let mut length = uniqed.len();
       assert_eq!(length,4)
   }
}
