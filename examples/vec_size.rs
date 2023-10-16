use std::mem::size_of;

use smallvec::SmallVec;

fn main() {
    let size = size_of::<Vec<i32>>();
    let small_vec = size_of::<SmallVec<[i32; 4]>>();
    println!("Vec: {}", size);
    println!("Small-vec: {}", small_vec);
}
