// For some use cases, when matching enums
// if let is cleaner for this use case and in addition allows various failure options to be specified
fn main() {

   let letter: Option<i32> = None;
   // If you need to specify a failure, use an else:
   if let Some(i) = letter {
      println!("Matched {:?}!", i);
   } else {
      // Destructure failed. Change to the failure case.
      println!("Didn't match a number. Let's go with a letter!");
   }

   enum Foo {
      Bar,
      Baz,
      Qux(u32)
   }

   let a = Foo::Bar;
   let b = Foo::Baz;
   let c = Foo::Qux(100);
   
   // Variable a matches Foo::Bar
   if let Foo::Bar = a {
      println!("a is foobar");
   }
   // Variable b does not match Foo::Bar
   // So this will print nothing
   if let Foo::Bar = b {
      println!("b is foobar");
   }
   // Variable c matches Foo::Qux which has a value
   // Similar to Some() in the previous example
   if let Foo::Qux(value) = c {
      println!("c is {}", value);
   }


}