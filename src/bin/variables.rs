// In most cases, the compiler will be able to infer the type of the variable from the context,

fn main() {
   let anint = 0u32;
   let _boolean = true;
   let _unit = ();
   
   let _copied_int = anint;
   
   // Variable bindings are immutable by default, but this can be overridden using the mut modifier.
   let mut mutable = 0;
   println!("{}", mutable);
   mutable = 42;
   println!("{}", mutable);
   
   // Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}.
   {
      // This binding only exists in this block
      let short_lived_binding = 2;
      println!("inner short: {}", short_lived_binding);
   } // Also, variable shadowing is allowed.

   // It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.
   let a : u32;
   if true { a = 11;} else { a = 22; } 
   println!("{}", a)
}