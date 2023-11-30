fn main() {
   log();
   is_even(2);
   
   apply(1, &mult);
}

// Unlike C/C++, there's no restriction on the order of function definitions
// Functions that "don't" return a value, actually return the unit type `()`
fn log() {
   println!("Hello World");
}

fn is_even(n : u32) -> bool { 
   n%2 == 0 // -> can omit return statement if last line is an expression (no ; at the end)
}

fn mult(n: u32 ) -> u32 {
   n*2
}

// Higher order function
fn apply(n: u32, f: &dyn Fn(u32) -> u32) -> u32{
   return f(n);
}

// Diverging functions never return. They are marked using !, which is an empty type.
fn _foo() -> ! {
   panic!("This call never returns.");
}
