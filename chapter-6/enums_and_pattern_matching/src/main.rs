#[derive(Debug)]
enum ThreeWayEither<T, U, V> {
    First(T),
    Second(U),
    Last(V),
}

#[derive(Debug)]
struct Zero {
    value: i64,
}

impl Zero {
    pub fn new(number: i64) -> Zero {
        match number {
            0 => Zero { value: 0 },
            t => panic!("Type Zero can not be {:?}", t),
        }
    }
}

#[derive(Debug)]
struct PositiveNumber {
    value: i64,
}

impl PositiveNumber {
    pub fn new(number: i64) -> PositiveNumber {
        match number {
            x if x > 0 => PositiveNumber { value: x },
            t => panic!("Type PositiveNumber can not be {:?}", t),
        }
    }
}

#[derive(Debug)]
struct NegativeNumber {
    value: i64,
}

impl NegativeNumber {
    pub fn new(number: i64) -> NegativeNumber {
        match number {
            x if x < 0 => NegativeNumber { value: x },
            t => panic!("Type NegativeNumber can not be {:?}", t),
        }
    }
}

trait Numbers {
    fn print_number(&self) -> String;
}

impl<'a> std::convert::From<&'a Zero> for String {
    fn from(num: &Zero) -> String {
        format!("{:?}", num)
    }
}

impl<'a> std::convert::From<&'a PositiveNumber> for String {
    fn from(num: &PositiveNumber) -> String {
        format!("{:?}", num)
    }
}

impl<'a> std::convert::From<&'a NegativeNumber> for String {
    fn from(num: &NegativeNumber) -> String {
        format!("{:?}", num)
    }
}

impl Numbers for Zero {
    fn print_number(&self) -> String {
        String::from(self)
    }
}

impl Numbers for PositiveNumber {
    fn print_number(&self) -> String {
        String::from(self)
    }
}

impl Numbers for NegativeNumber {
    fn print_number(&self) -> String {
        String::from(self)
    }
}

fn main() {
    let zero: ThreeWayEither<Zero, PositiveNumber, NegativeNumber> =
        ThreeWayEither::First(Zero::new(0));
    let positive_number: ThreeWayEither<Zero, PositiveNumber, NegativeNumber> =
        ThreeWayEither::Second(PositiveNumber::new(123));
    let negative_number: ThreeWayEither<Zero, PositiveNumber, NegativeNumber> =
        ThreeWayEither::Last(NegativeNumber::new(-123));

    print_type(zero);
    print_type(positive_number);
    print_type(negative_number);
}

fn print_type(num: ThreeWayEither<Zero, PositiveNumber, NegativeNumber>) {
    match num {
        ThreeWayEither::First(number) => println!("Zero {}", number.value),
        ThreeWayEither::Second(number) => println!("Positive Number {}", number.value),
        ThreeWayEither::Last(number) => println!("Negative Number {}", number.value),
    }
}
