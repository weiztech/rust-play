fn main() {
    let numbers: [u64; 4] = [0b1011, 0b1101, 0b1100, 0b1111111111111111];

    for number in numbers.iter() {
        println!("The parity of {} {:b} {:X} is {}", number, number, number, parity(*number));
    }
    let mut num = 11;
    println!("Before num {} {}", num, num << 1);
    num ^= num << 1;
    println!("After num {} {:b} {}", num, num, num & 1);
}

fn parity(x: u64) -> u64 {
    let mut y = x;
    y ^= y >> 32;
    y ^= y >> 16;
    y ^= y >> 8;
    y ^= y >> 4;
    y ^= y >> 2;
    y ^= y >> 1;
    y & 1
}