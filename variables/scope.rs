fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        println!("The value of x is {} and the value of y is {}", x, y);
    }
    println!("The value of x is {} and the value of y is {}", x, y);
}
