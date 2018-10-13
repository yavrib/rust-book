use std::collections::HashMap;

fn divisible_by_three(number: i32) -> bool {
    number % 3 == 0
}

fn divisible_by_five(number: i32) -> bool {
    number % 5 == 0
}

fn main() {
    let number = 15i32;
    let v: Vec<fn(i32) -> bool> = vec![divisible_by_five, divisible_by_three];

    let results: Vec<bool> = v
        .into_iter()
        .map(|f: fn(i32) -> bool| {
            let result = f(number);
            println!("{:?}", result);
            result
        }).collect();

    match results.get(0) {
        Some(_) => println!("{:?} is divisible by 3!", number),
        None => println!("Please use a number!"),
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    println!("{:?}", s3);

    let mut person = HashMap::new();

    let symbol_for_name = String::from("Name");

    person.insert(symbol_for_name.clone(), String::from("Berkan")); // HashMap takes ownership
    person
        .entry(symbol_for_name.clone())
        .or_insert(String::from("Man-without-a-name")); // runs if there is no entry called "Name"

    println!("{:?}", person.get(&symbol_for_name));

    let fox = "Quick brown fox jumps over the lazy dog.";

    let mut fox_map = HashMap::new();

    for word in fox.split_whitespace() {
        let count = fox_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", fox_map);
}
