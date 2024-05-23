use std::fmt;

#[derive(Debug)]
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), //Lat and Long
}
//This is demonstrating how to create your own 'display trait'
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
//This shows how to use the Display trait on an enum
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Unknown => write!(f, "The location is unknown"),
            Location::Anonymous => write!(f, "The location is anonymous"),
            Location::Known(la, lg) => {
                write!(f, "The latitude is {} and the longitude is {}", la, lg)
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

    let address = Location::Unknown;
    println!("{}", address);
    let address = Location::Anonymous;
    println!("{}", address);
    let address = Location::Known(23.3354, -80.098);
    println!("{}", address);
}
