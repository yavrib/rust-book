fn main() {
    let x = 5i64;
    let _y = x; // copies value of x because it's simple to copy an integer

    let hello = String::from("hello");
    let another_hello = hello;

    /* println!("{}", hello); */

    /*
        above code does not work because Strings
        are not easy to copy and the ownership of
        "hello" has been moved from hello to another_hello.
    */

    let yet_another_hello = another_hello.clone();

    /*
        above code works because this time we
        explicitly clone the String ourselves.
        Rust does not assume you would want this as default.
    */

    println!("{}", yet_another_hello); // prints "hello"
    println!("{}", another_hello); // prints "hello"
    println!("{}", x); // prints 5

    let hello_world = String::from("Hello, world!");

    print(hello_world);

    /*
        println!("{}", hello_world);
    */

    /*
        above code does not work because the ownership has
        been transferred to print function. Furthermore, once
        the function is executed, values within the function
        scope will be deallocated. Therefore our string will
        be dropped from memory.
    */

    let yet_another_hello_world = String::from("Hello, world!");

    print_with_borrow(&yet_another_hello_world);

    /*
        above code works because we now "borrow" yet_another_hello_world.
        Once the function is executed, yet_another_hello_world value will
        return to its original owner.
    */

    /*
        Ownership Rules:
        - Each value in Rust has a vareiable that's called its owner.
        - There can only be one owner at a time.
        - When the over goes out of the scope, the value will be dropped.

        Copy Rules:
        Simple Scalar values can be copied without using copy().
        - integer types, such as u32
        - boolean type, bool
        - floating point types, such as f64
        - character type, char
        - tuples only if they contain above types

        Reference Rules:
        - At any given time, either one mutable reference or any number of
        immutable references may exist for a value.
        - References must always be valid.
    */
}

fn print(s: String) {
    println!("{}", s);
}

fn print_with_borrow(s: &String) {
    println!("{}", s);
}
