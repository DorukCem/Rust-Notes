fn analyze_slice(slice: &[i32]) {
   println!("First element of the slice: {}", slice[0]);
   println!("The slice has {} elements", slice.len());
}

fn main() {
   // array of data type i32 with length 5
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
   println!("The length of the array: {}", arr.len());

   // Arrays can be automatically borrowed as slices.
   println!("Borrow the whole array as a slice.");
   analyze_slice(&arr);

   // Slices can point to a section of an array.
   // They are of the form [starting_index..ending_index].
   println!("Borrow a section of the array as a slice.");
   analyze_slice(&arr[1 .. 4]);

   // Arrays can be safely accessed using `.get`, which returns an `Option`.
   for i in 0..arr.len() + 1 { // Oops, one element too far!
      match arr.get(i) {
         Some(val) => println!("{}: {}", i, val),
         None => println!("Slow down! {} is too far!", i),
      }
   }
}

// Arrays are stack allocated.
// An array is a collection of objects of the same type T, stored in contiguous memory.