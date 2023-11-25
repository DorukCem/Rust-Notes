use std::convert::From;
#[allow(dead_code)]

// We can do similar for defining a conversion for our own type
struct Number {
   value: i32,
}
impl From<i32> for Number {
   fn from(item: i32) -> Self {
      Number { value: item }
   }
}

//if you have implemented the From trait for your type, Into will call it when necessary.
struct Num {
   _value: i32,
}
impl Into<Num> for i32 {
   fn into(self) -> Num {
      Num { _value: self }
   }
}

// TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
   type Error = ();
   
   fn try_from(value: i32) -> Result<Self, Self::Error> {
      if value % 2 == 0 {
         Ok(EvenNumber(value))
      } else {
         Err(())
      }
   }
}


fn main() {
   // str to string
   let my_str = "hello";
   let _my_string = String::from(my_str);
   
   let _n = Number::from(30);

   let int = 5;
   let _num: Num = int.into();
}