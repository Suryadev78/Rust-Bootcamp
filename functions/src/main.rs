fn main() {
    let a = 23;
    let b = 24;
    check_even_odd(a, b);
    let square_a = square(a);
    println!("Square of {} is {}", a, square_a);
}

fn check_even_odd(a: i32, b: i32) {
    if a % 2 == 0 {
        println!("{} is even", a);
    } else {
        println!("{} is odd", a);
    }
    if b % 2 == 0 {
        println!("{} is even", b);
    } else {
        println!("{} is odd", b);
    }
}

fn square(a: i32) -> i32 {
    a * a
}
