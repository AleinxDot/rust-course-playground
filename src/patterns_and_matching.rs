pub fn patterns(){
    // match Arms
    let x   = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    // let Statements
    let x = 5;
    println!("The value of x is: {}", x);

    // let statements can also be used to destructure tuples
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Using let...else blocks to handle Option values
    let some_option_value = Some(5);
    let Some(x) = some_option_value else {
        println!("No value");
        return;
    };

    println!("x: {}", x);


}
pub fn matching_names_var() {
    let x = Some(5);
    let y = 10;

    // We have a different scope inside so the value of y is not 10
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

pub fn matching_multiple_patterns() {
    let x = 1;

    match x {
        //We can use 1 or 2
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
pub fn matching_ranges_with_periods() {
    let x = 5;

    match x {
        //We can use a range to compare the value of x
        1..=5 => println!("between 1 and 5"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub fn matching_structs() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
pub fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}