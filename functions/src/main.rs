fn main() {
    let a = 5;
    let b = 10;
    let result = add(a,b);
    let subtraction = subtract(a,b);
    println!("Result: {}", result);
}
fn add(a:i32,b:i32)->i32{
    return a+b;
}
fn subtract(a:i32,b:i32)->i32{
    return a-b; }
