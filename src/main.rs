use std :: io;
fn main() {
    println!("Please input the integer");
    let mut num = String :: new();
    io :: stdin()
        .read_line(&mut num)
        .expect("Failed to read line.");
    
    let n:i32 = num.trim().parse().expect("Error");

    for i in 0..n{
         for _j in 0..i+1 {
             print!("*");
         }
         println!("");
    }

}
