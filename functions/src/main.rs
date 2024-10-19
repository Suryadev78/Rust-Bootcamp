fn main() {
    let a = 23;
    let b = 24;
   let isEven = IsEven(a,b);
   let square = square(a);
}
fn IsEven(a:i32,b:i32){
    if(a%2==0){
        println!("{} is even",a);
    }
    else{
        println!("{} is odd",a)
    }
    if (b%2==0){
    println!("{} is even",b);
   }
   else {
    println!("{} is odd",b);

   }
}
fn square(a:i32){
    return a*a;
}
