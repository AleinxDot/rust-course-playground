fn variables(){
    let mut x = 10;
    x = x + 1;
    println!("{}", x);
}
pub fn main(){
    print_variables();
}
fn print_variables(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    println!("Hello, world!");
}
