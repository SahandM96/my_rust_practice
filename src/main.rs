fn main() {


    let b = highest(56 ,12,35);
    println!("Hello, Friend , I\'m using Rust !");
    println!("highest ==> {}",b);
}



fn highest(a : i32 , b : u32, c : i8) -> i32 {
    let mut res = a ;

    if b as i32>res{
        res = b as i32;
    }

    if c as i32 > res {
        res = b as i32;
    }
   return res;
}