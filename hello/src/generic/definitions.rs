use std::fmt; // Import `fmt`

pub struct Point<T> {
    x: T,
    y: T,
}

/// Implement Diplay trait
/// impl<T> states that the following is an implementation for something generic (just as Point)
///where T: std::fmt::Display requires that T implements Display. This is needed because you want to write self.class, which is of type T
/// 
impl <T>fmt::Display for Point<T>
    where T: fmt::Display,
    {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub trait Distance {
    fn distance_from_origin(&self) -> f32;
}
impl Distance for Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn use_struct() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4 };
    println!(" both_integer {}", both_integer);
}


//enums
enum Option<T> {
    Some(T),//variant with some valu of type T
    None, //no values
}

enum Result<T, E> {
    Ok(T), //ok with value
    Err(E), //ko with error
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn test_struct_methods() {
    let p = Point { x: 5, y: 10 };
    println!("p {}", p);
    println!("p.x = {}", p.x());
}