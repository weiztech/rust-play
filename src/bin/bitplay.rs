fn calculate(num: u32, filter: &mut u32) -> u32 {
    let value = 1 << num;
    let xor = *filter ^ value;
    println!("1 << {} = {} , the bit {:b}", num, value, value );
    println!("{} XOR {} = {}", filter, value, xor);
    println!("Bit {:b} XOR {:b} = {:b}", filter, value, xor);
    println!("Total Filter {}\n", xor.count_ones());
    xor
}

fn main() {
    let x = 12; // In binary: 1100
    let y = x << 1; // left shift by 1 position: 11000 (24 in decimal)
    let z = x >> 1; // Right shift by 1 position: 0110 (6 in decimal)
    println!("x value {} - {:b}", x, x);
    println!("y value {} - {:b}", y, y);
    println!("z value {} - {:b}", z, z);

    let binary_string = format!("{:b}", x);
    println!("Binary string {binary_string}");
    let bits: Vec<u8> = binary_string.chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect();

    println!("The binary representation of {} is {:?}", x, bits);

    let c: char = 'a';
    println!("Test char `{}` ({}) to digit --> {:?} \n", c, c as u8, c.to_digit(16));

    let mut filter = 0u32;
    /*for num in 1..6{
        filter = calculate(num, &mut filter);
    }*/
    println!("Test Power {}", 2u8.pow(3));
    let test_xor = 1 << 2;
    println!("1 XOR 2 = {} {:b}", test_xor, test_xor);
    let test_2_xor = 1 << test_xor;
    println!("1 XOR {} = {} {:b}\n", test_xor, test_2_xor, test_2_xor);

    let data = [
        98, 108, 109, 110, 111, 99, 100, 101, 102, 103, 104, 106, 107, 105, 108, 109
    ];
    let mut value = 0u32;
    for i in data{
        let bit_value = i % 32;
        println!("Real Value {}, bit 32 - value {}", i, bit_value);
        value = calculate(bit_value, &mut value);
    }


    const READ_PERMISSION: u8 = 0b0000_0001;
    const WRITE_PERMISSION: u8 = 0b0000_0010;
    const EXECUTE_PERMISSION: u8 = 0b0000_0100;
    let user_permission: u8 = READ_PERMISSION | WRITE_PERMISSION;
    println!("{READ_PERMISSION} {WRITE_PERMISSION} {EXECUTE_PERMISSION}");
    println!("Permission: {:b} {:b} {:b}", READ_PERMISSION, WRITE_PERMISSION, EXECUTE_PERMISSION);
    println!("Permission: {} {} {} {}", READ_PERMISSION, WRITE_PERMISSION, EXECUTE_PERMISSION, 0b0000_1000);
    println!("{} {:b}", user_permission, user_permission);
    println!("{}", user_permission & EXECUTE_PERMISSION);
    println!("{}", user_permission & (READ_PERMISSION | WRITE_PERMISSION));
    println!("{} {}", user_permission & READ_PERMISSION, user_permission & WRITE_PERMISSION);
    println!("{}", (user_permission & READ_PERMISSION | user_permission & WRITE_PERMISSION));
}