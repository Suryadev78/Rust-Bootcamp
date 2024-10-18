fn main() {
    let a = 23;
    let b = 24;
   let isEven = IsEven(a,b);
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
