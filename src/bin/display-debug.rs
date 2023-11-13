use std::fmt; // Import `fmt`
// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Structure(i32);


#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Point2D {
   // This trait requires `fmt` with this exact signature.
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      
      write!(f, "x: {}, y: {}", self.x, self.y)
   }
}

fn main() {

   println!("Debug will print: {:?}", Structure(3));

   let point = Point2D { x: 3.3, y: 7.2 };

   println!("Display: {}", point);
   println!("Debug: {:?}", point);


}