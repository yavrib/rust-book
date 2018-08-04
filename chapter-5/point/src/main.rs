#[derive(Debug, Clone)] // Add Debug to be able to print every key and value. Add Clone to be able to clone the values within the Point
struct Point {
    x: f64,
    y: f64,
}
// Custom type Point

#[derive(Debug, Clone)]
struct Rectangle {
    first_edge: Point,
    second_edge: Point,
    third_edge: Point,
    fourth_edge: Point,
}
// Custom type Rectangle

impl Rectangle {
    fn center_of_mass(&self) -> Point {
        let x_coord: f64 =
            (self.first_edge.x + self.second_edge.x + self.third_edge.x + self.fourth_edge.x)
                / 4f64;
        let y_coord: f64 =
            (self.first_edge.y + self.second_edge.y + self.third_edge.y + self.fourth_edge.y)
                / 4f64;
        return Point {
            x: x_coord,
            y: y_coord,
        };
    }
}
// Implementation of center_of_mass

impl Rectangle {
    fn new(
        first_egde: Point,
        second_edge: Point,
        third_edge: Point,
        fourth_edge: Point,
    ) -> Rectangle {
        Rectangle {
            first_edge: first_egde,
            second_edge: second_edge,
            third_edge: third_edge,
            fourth_edge: fourth_edge,
        }
    }
}
// Implementation of new

fn main() {
    let r = Rectangle {
        first_edge: Point { x: 0f64, y: 0f64 },
        second_edge: Point { x: 1f64, y: 1f64 },
        third_edge: Point { x: 2f64, y: 2f64 },
        fourth_edge: Point { x: 3f64, y: 3f64 },
    };

    let yet_another_r = Rectangle {
        first_edge: Point { x: 0f64, y: 7f64 },
        ..r.clone() // Rest of the fields are filled with r's clones of the same name fields. No data is harmed in this practice.
    };

    let yet_another_yet_another_r = Rectangle::new(
        Point { x: 0f64, y: 0f64 },
        Point { x: 0f64, y: 0f64 },
        Point { x: 0f64, y: 0f64 },
        Point { x: 0f64, y: 0f64 },
    );

    println!(
        "Point of {:?} has {:?} as center of mass",
        r,
        r.center_of_mass()
    );

    println!(
        "Point of {:?} has {:?} as center of mass",
        yet_another_r,
        yet_another_r.center_of_mass()
    );

    println!(
        "Point of {:?} has {:?} as center of mass",
        yet_another_yet_another_r,
        yet_another_yet_another_r.center_of_mass()
    );
}
