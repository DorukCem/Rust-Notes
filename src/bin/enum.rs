#![allow(dead_code)]

// An enum in Rust is a type that represents data that is one of several 
// possible variants. Each variant in the enum can optionally have data associated with it:

// An enum is a single type. A value of the enum can match any of the variants. 
// For this reason, an enum is sometimes called a ‘sum type’:

// A value of an enum type contains information about which variant it is, 
// in addition to any data associated with that variant.

// Rust version of std::variant

enum WebEvent {
   // An `enum` variant may either be `unit-like`,
   PageLoad,
   PageUnload,
   // like tuple structs,
   KeyPress(char),
   Paste(String),
   // or c-like structures.
   Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
   match event {
       WebEvent::PageLoad => println!("page loaded"),
       WebEvent::PageUnload => println!("page unloaded"),
       // Destructure `c` from inside the `enum` variant.
       WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
       WebEvent::Paste(s) => println!("pasted \"{}\".", s),
       // Destructure `Click` into `x` and `y`.
       WebEvent::Click { x, y } => {
           println!("clicked at x={}, y={}.", x, y);
       },
   }
}

// C like enums
enum Number {
   Zero,
   One,
   Two,
}
enum Color {
   Red = 0xff0000,
   Green = 0x00ff00,
   Blue = 0x0000ff,
}
fn main() {
   let pressed = WebEvent::KeyPress('x');
   let pasted  = WebEvent::Paste("my text".to_owned());
   let click   = WebEvent::Click { x: 20, y: 80 };
   let load    = WebEvent::PageLoad;
   let unload  = WebEvent::PageUnload;

   inspect(pressed);
   inspect(pasted);
   inspect(click);
   inspect(load);
   inspect(unload);

   // `enums` can be cast as integers.
   println!("zero is {}", Number::Zero as i32);
   println!("one is {}", Number::One as i32);

   println!("roses are #{:06x}", Color::Red as i32);
   println!("violets are #{:06x}", Color::Blue as i32);
}