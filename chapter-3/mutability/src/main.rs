fn main() {
    let x = 5; // Immutable by default
    let mut y = 5; // Mutable

    println!("x: {}, y: {}", x, y);

    let x = x + 1; // Shadowed

    println!("shadowed x: {}", x);

    y = 6; // Mutated

    println!("new y: {}", y);
}
