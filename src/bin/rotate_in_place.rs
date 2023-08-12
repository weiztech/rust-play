fn rotate_left(arr: &mut [i32], k: usize) {
    let n = arr.len();
    let k = k % n;
    arr[..k].reverse();
    println!("Arr 1 {:?}", arr);
    arr[k..].reverse();
    println!("Arr 2 {:?}", arr);
    arr.reverse();
    println!("Arr 3 {:?}", arr);
}

fn main() {
    let mut data = [1, 2, 3, 4, 5, 6, 7];
    rotate_left(&mut data, 3);
    println!("{:?}", data); // prints [4, 5, 6, 7, 1, 2, 3]
}