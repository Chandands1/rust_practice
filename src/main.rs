fn main() {
    println!("Hello World!");
    add(5, 6);
}
fn add(number1: i32, number2: i32) {
    let c: i32 = number1 + number2;
    println!("The addition of {number1} and {number2} is {c}");
}
