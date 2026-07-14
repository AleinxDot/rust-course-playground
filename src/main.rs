use std::io;
// use std::collections::HashSet;
// use std::collections::HashMap;
use variables::print_variables;

mod variables;
mod patterns_and_matching;

fn main() {

    //Rust has auto-inference
    //String reserve name &str
    let my_string: &str = "Hello, world!";
    println!("{}", my_string);

    //Int reserve name is :i32
    let mut my_int: i32 = 5;
    my_int += 4;
    println!("{}", my_int);

    //Float reserve name is :f64
    let my_float: f64 = 3.14;
    println!("{}", my_float);
    // my_float = my_float + 1.0; // This line would cause an error because my_float is not mutable

    //Booleans reserve name bool
    let my_bool: bool = true;
    println!("{}", my_bool);

    //Constants
    const MY_CONSTANT: i32 = 100;
    println!("{}", MY_CONSTANT);

    //Control de flujo

    if my_bool == true {
        println!("The value of my_int is: {}", MY_CONSTANT);
    }
    else {
        println!("The value of my_int is not: {}", MY_CONSTANT);
    }

    // List

    // let mut my_list: Vec<&str> = vec![my_string,"Hello"];
    // my_list.push("world");
    // println!("{}", my_list.join(""));
    // println!("my_list[0]: {}", my_list[0]);

    // //Sets
    //
    // let mut my_set: HashSet<&str> = vec!["Hello", "world"].into_iter().collect();
    // my_set.insert("Hello");
    // println!("my_set: {:?}", my_set);
    //
    // //Maps
    // let mut my_map: HashMap<&str, i32> = vec![("Hello", 1), ("world", 2)]
    //     .into_iter()
    //     .collect();
    // my_map.insert("Hello", 3);
    // println!("my_map: {:?}", my_map);
    //
    // //Bucles
    // // For
    // for (key, value) in &my_map{
    //     println!("{}: {}", key, value);
    // }
    // While
    // while let Some(value) = my_map.get("Hello") {
    //     println!("{}", value);
    // }
    //Function
    print();
    print_variables();

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    // We use .expect to give an error message
    // Also .parse to convert the input in the variable type that is expected
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    //Patterns
    patterns_and_matching::patterns();
    patterns_and_matching::matching_names_var();
    patterns_and_matching::matching_multiple_patterns();
    patterns_and_matching::matching_ranges_with_periods();
    patterns_and_matching::matching_structs();

}
fn print(){
    println!("Hello, world!");
    println!("Rem hijo de la remil");
}

