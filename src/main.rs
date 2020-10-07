fn main() {


    let b = highest(56 ,12,35);
    let re = plus_fun(2,2);
    println!("Hello Friend , I\'m using Rust !");
    println!("highest ==> {}",b);
    println!("plus ==> {}",re);
    loop_to_10();
    loop_to_10_with_while();
    loop_to_10_with_for();
    loop_in_array();
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
fn loop_to_10_with_while(){
    let mut counter=0;

    while counter <10 {
        counter+=1;
        println!("while ==> {}",counter);
    }
}
fn loop_to_10_with_for(){
    for n in 0..10{
        if n==6 {
            break;
        }
        println!("for ==> {}",n);
    }
}

fn loop_in_array(){
    let mut ve =Vec::new();
    ve.push(6);
    ve.push(16);
    ve.push(75);

    for item in ve{
        print!("{}-",item)
    }

}