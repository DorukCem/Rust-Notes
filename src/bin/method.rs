#[allow(dead_code)]
struct Point {
   x: f64,
   y: f64,
}
#[allow(dead_code)]
// Implementation block, all `Point` associated functions & methods go in here
impl Point {
   // Associated functions don't need to be called with an instance.
   // These functions are generally used like constructors.
   fn origin() -> Point {
      return Point { x: 0.0, y: 0.0 }
   }

   // Another associated function, taking two arguments:
   fn new(x: f64, y: f64) -> Point {
      return Point { x: x, y: y }
   }

   fn add_velocity(&mut self, vx: f64, vy: f64)
   {
      self.x += vx;
      self.y += vy;
   }

   fn print(&self) {
      println!("{}, {}", self.x, self.y)
   }
}

fn main() {
   let mut p = Point{ x: 0.1, y: 0.2 };
   let _p_on_origin = Point::origin();
   let _p2 = Point::new(0.3, 0.5);
   p.print();
   p.add_velocity(0.5, 0.8);
   p.print();
}