fn main() {
    println!("{}", this_knows_what_to_say());
    println!("{}", this_returns_a_string());

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

    let found_item: &str = match find_first(months.to_vec(), |&month| month == "November") {
        Some(item) => item,
        None => panic!("Item not found!"),
    };

    println!("{}", found_item);

    println!("{:?}", months);

    let another_found_item: &str = match find_first(months.to_vec(), |&month| month == "Novembre") {
        Some(item) => item,
        None => panic!("Item not found!"),
    };

    println!("{}", another_found_item);
}

fn this_knows_what_to_say() -> &'static str {
    "Hello, world!"
}

fn this_returns_a_string() -> String {
    String::from("Hello, world!")
}

// let's implement a find function
fn find_first<T: PartialEq + Clone>(collection: Vec<T>, query: fn(&T) -> bool) -> Option<T> {
    if collection.is_empty() {
        return None;
    }

    let (first, rest) = get_first_and_rest(&collection);
    match first {
        Some(first) => {
            if query(first) {
                // ¯\_(ツ)_/¯
                return Some(first.clone());
            }
        }
        None => return None,
    }

    // ¯\_(ツ)_/¯
    return find_first((*rest).to_vec(), query);
}

// Well apparently collection and the elements have to live long enough so they all share the same lifetime.
fn get_first_and_rest<'a, T: 'a>(collection: &'a Vec<T>) -> (Option<&'a T>, &'a [T]) {
    let first: Option<&'a T> = collection.first();
    let rest: &'a [T] = &collection[1..];
    return (first, rest);
}

// Even though the functionality of my find_first is
// quite redundant, this was a great help on grasping
// (even though we are not there yet) lifetimes.
