#![allow(dead_code)]

struct Unit;

// A tuple struct
struct Pair(i32, f32);

struct  Point {
   x: f32,
   y: f32,
}

struct Person {
   name: String,
   age: u8,
}

// Structs can be reused as fields of another struct
struct Rectangle {
   top_left: Point,
   bottom_right: Point,
}

fn main() {

   let point = Point{x: 10.3, y: 0.4};
   // Access the fields of the point
   println!("point coordinates: ({}, {})", point.x, point.y);

   let _rectangle = Rectangle {
      // struct instantiation is an expression too
      top_left: Point { x: 2f32, y: 3f32 },
      bottom_right: point,
  };

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);
  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);
}