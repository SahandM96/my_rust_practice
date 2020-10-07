fn main() {


    let b = highest(56 ,12,35);
    let re = plus_fun(2,2);
    println!("Hello Friend , I\'m using Rust !");
    println!("highest ==> {}",b);
    println!("plus ==> {}",re);
    loop_to_10();
    loop_to_10_with_for()
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

fn plus_fun(first:i32 ,second:i32)->i32{
    let result;
    result = first  +second;
    result
}

fn loop_to_10(){
    let mut counter=0;

    loop{
        counter+=1;
        println!("loop ==> {}",counter);
        if counter>=10 {
            return;
        }
    }
}
fn loop_to_10_with_for(){
    for n in 0..10{
        println!("for ==> {}",n);
    }
}