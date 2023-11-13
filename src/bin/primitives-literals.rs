// * Scalar Types
// Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// Floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false
// The unit type (), whose only possible value is an empty tuple: ()
// *****************

// * Compound Types
// Arrays like [1, 2, 3]
// Tuples like (1, true)


fn main() {
   // Variables can be type annotated.
   let _logical: bool = true;

   let _a_float: f64 = 1.0;  // Regular annotation
   let _an_integer   = 5i32; // Suffix annotation

   // Or a default will be used.
   let _default_float   = 3.0; // `f64`
   let _default_integer = 7;   // `i32`

   // A type can also be inferred from context.
   let mut _inferred_type = 12; // Type i64 is inferred from another line.
   _inferred_type = 4294967296i64;

   // We can tell the compiler the type of the literals we use. 
   println!("1 + 2 = {}", 1u32 + 2);
   // Use underscores to improve readability!
   println!("One million is written as {}", 1_000_000u32);

}