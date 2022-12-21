mod new_in_rust;
mod cnn;

fn main () {
    let mut arr: [i32; 4] = [4, 6, 8, 10];

    for x in 0..arr.len() {
        arr[x] = arr[x].pow(x as u32) as i32;
        println!("x: {}", arr[x]);
    }
}