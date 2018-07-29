fn main() {
    let _from_int_to_float: f64 = 2i32.into(); // from integer 32-bit to float 64-bit

    let tup = (500, 6.4, 1);

    let (_x, _y, _z) = tup; // destructuring? yes

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = 0;

    println!("First month is: {}", months[first])
}
