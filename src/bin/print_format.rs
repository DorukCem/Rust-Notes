fn main() {
   
   println!("{} days", 31);
   
   //you can directly capture the argument from a surrounding variable
   let number: i32 = 2;
   println!("{number} is in this scope");

   // Specifying an integer inside `{}`
   // determines which additional argument will be replaced
   println!("{0} bought a gift for {1}. {1} thanked {0} for the gift", "Walter", "Hank");

   // Different formatting can be invoked by specifying a format character after ':'
   println!("{0} in binary is {0:b}", 42);
}