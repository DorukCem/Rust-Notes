fn main() {
   //Rust provides a loop keyword to indicate an infinite loop. 
   let mut count = 0;
   loop {
      count += 1;
      print!(" {}", count);
      
      if count == 5 {break;}
   }
   
   // It's possible to break or continue outer loops when 
   //  dealing with nested loops. In these cases, the loops must 
   //  be annotated with some 'label, and the label must be passed to the break/continue statement.
   #[allow(unused_labels)]

   'outer: loop {
      'inner: loop {
         break 'outer;
      }
   }

   // Return from loop is possible
   let mut counter = 0;

   let result = loop {
      counter += 1;

      if counter == 10 {
         break counter * 2;
      }
   };
   println!("\n{}",result);

   // * while
   while count < 20 {
      count += 1;
   }

}