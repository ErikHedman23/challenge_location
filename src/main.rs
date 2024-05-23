#[derive(Debug)]
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("The location is unknown"),
            Location::Anonymous => println!("The location is Anonymous"),
            Location::Known(la, lg) => {
                println!("{} is the latitude, and {} is the longitude", la, lg)
            }
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(23.3354, -80.098);
    address.display();
}
