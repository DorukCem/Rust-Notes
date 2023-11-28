fn main() {
   log();
   is_even(2);
}

// Unlike C/C++, there's no restriction on the order of function definitions
// Functions that "don't" return a value, actually return the unit type `()`
fn log() {
   println!("Hello World");
}

fn is_even(n : u32) -> bool { 
   n%2 == 0 // -> can omit return statement if last line is an expression
}

