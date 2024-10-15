// fn main() {
//     let mut  x:i8 = 10;
//     let b:u32 = 20;
//     let c:f32 = 30.0;
//     for i in 0..1000 {
//         x= x + 100;
//     }
//     println!("here's your number {}",x);
//    println!("here's your number {} {}",b, c);
// }
// fn main(){
//     let is_male = true;
//     let is_above = true;
//     if is_male {
//         println!("You are a male");
//     }
//     else{
//         println!("you are not male")
//     }
//     if is_male && is_above{
//         println!("you are are a male with age above 18")
//     }
// }
// fn main (){
//     let x:i32 = 10;
//     let y:i32 = 1;
//     if x % 2 == 0 {
//         println!("x is even");
//     }
//     else{
//         println!("x is odd");
//     }
//     if y % 2 == 0 {
//         println!("y is even");
//     }
//     else{
//         println!("y is odd");
//     }

// }
fn main() {
    let a: i32 = 10;
    let b: i32 = 15;
    let sum: i32 = add(a, b);
    let sub: i32 = sub(a, b);
    println!("sum of {} and {} is {}", a, b, sum);
    println!("sub of {} and {} is {}", a, b, sub);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}
