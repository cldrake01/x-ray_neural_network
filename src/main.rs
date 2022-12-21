use std::cmp::max;

mod new_in_rust;
mod cnn;

fn main () {
    let mut arr: [i32; 4] = [4, 6, 8, 10];

    for num in 0..arr.len() {
        arr[num] = re_lu(arr[num]);
        println!("{}: {}", num, arr[num]);
    }

    fn re_lu(x: i32) -> i32 {
        max(0,x)
    }

    fn tanh(x: i32) -> i32 {
        x
    }
}