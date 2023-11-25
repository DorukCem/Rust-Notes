fn main() {
   // The for in construct can be used to iterate through an Iterator. 
   // One of the easiest ways to create an iterator is to use the range notation a..b.
   // This yields values from a (inclusive) to b (exclusive) in steps of one
   for i in 1..12 {
      print!(" {}", i);
   }
   // Alternatively, a..=b can be used for a range that is inclusive on both ends.

   // iter - This borrows each element of the collection through each iteration. 
   // Thus leaving the collection untouched and available for reuse after the loop.
   let names = vec!["Bob", "Frank", "Ferris"];

   for name in names.iter() {
      match name {
         &"Ferris" => println!("There is a rustacean among us!"),
         _ => println!("Hello {}", name),
      }
   }

   // into_iter - This consumes the collection so that on each iteration 
   // the exact data is provided. Once the collection has been consumed 
   // it is no longer available for reuse as it has been 'moved' within the loop
   for name in names.into_iter() {
      match name {
         "Ferris" => println!("There is a rustacean among us!"),
         _ => println!("Hello {}", name),
      }
   }

   // iter_mut - This mutably borrows each element of the collection,
   // allowing for the collection to be modified in place.
   let mut names = vec!["Bob", "Frank", "Ferris"];

   for name in names.iter_mut() {
      *name = match name {
         &mut "Ferris" => "There is a rustacean among us!",
         _ => "Hello",
      }
   }

}