fn main() {
   let num = 42;

   match num {
      1 => println!("One"),
      2 | 3 | 5 | 7 | 11 => println!("This is a prime"), // match several values
      13..=19 => println!("A teen"), // match a range
      _ => println!("This number is not special")
   }

   let tuple = (0, -2, 3);
   match tuple {
      (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
      (1, ..)  => println!("First is `1` and the rest doesn't matter"),
      (.., 2)  => println!("last is `2` and the rest doesn't matter"),
      (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
      _      => println!("It doesn't matter what they are"),
   }

   let array = [1, 3, 5, 7, 13];
   match array {
      [3, _, tail @ ..] => println!("The tail is {tail:?}"),
      [first, middle @ .., last] => println!( "first = {}, middle = {:?}, last = {}", first, middle, last),
   }

   #[allow(dead_code)]
   enum Color {
      Red,
      Blue,
      RGB(u32, u32, u32),
      
   }
   let color = Color::RGB(122, 17, 40);
   match color {
      Color::Red   => println!("The color is Red!"),
      Color::Blue  => println!("The color is Blue!"),
      Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
   }

   let reference = &4;
   
   match reference {
      &val => println!("Got a value via destructuring: {:?}", val),
   }
   match *reference {
      val => println!("Got a value via destructuring: {:?}", val),
   }

   struct Foo {
      x: (u32, u32),
      y: u32,
   }
   let foo = Foo { x: (1, 2), y: 3 };

   match foo { 
      Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
      Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
      Foo { y, .. } => println!("y = {}, we don't care about x", y),
   }

   // * GUARDS ----------------------
   #[allow(dead_code)]
   enum Temperature {
      Celsius(i32),
      Fahrenheit(i32),
   } 
   let temperature = Temperature::Celsius(35);
   match temperature {
      Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
      Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

      Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
      Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
   }
   // ----------------------------

   fn age() -> u32 {
      15
   }
   match age() {
      0 => println!("I haven't celebrated my first birthday yet"),

      n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
      n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
      n => println!("I'm an old person of age {:?}", n),
   }

}