// Rust provides no implicit type conversion (coercion) between primitive 
// types. But, explicit type conversion (casting) can be performed using the as keyword.
#![warn(dead_code)]

fn main() {
   let decimal = 65.4321_f32;
   let integer = decimal as u8;

   println!("Casting: {} -> {}", decimal, integer);

   // Unsuffixed literals, their types depend on how they are used
   let _i = 1;
   let _f = 1.0;
   // The type inference engine is pretty smart. It does more than looking at the type 
   // of the value expression during an initialization. It also looks at how the variable 
   // is used afterwards to infer its type. Here's an advanced example of type inference:
   
   let elem = 5u8;
   let mut vec = Vec::new();
   // At this point the compiler doesn't know the exact type of `vec`, it
   // just knows that it's a vector of something (`Vec<_>`).
   vec.push(elem);

   // `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
   type NanoSecond = u64;

   let nanoseconds: NanoSecond = 5;
   println!("{} nanoseconds", nanoseconds,);

}