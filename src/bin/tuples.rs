
// A tuple is a collection of values of different types

fn main() {
   let tup = (12, 'a', true);

   println!("The values inside the tuple are: {}, {}, {}", tup.0, tup.1, tup.2);

   println!("Tuples are printable: {:?}", tup);

   // To create one element tuples, the comma is required to tell them apart
   // from a literal surrounded by parentheses.
   println!("One element tuple: {:?}", (5u32,));

   let tuple = (1, "hello", 4.5, true);
   let (a, b, c, d) = tuple;
   println!("Tuples can be unpacked: {:?}, {:?}, {:?}, {:?}", a, b, c, d);

}