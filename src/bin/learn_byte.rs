pub fn benny(input: &[u8]) -> Option<usize> {
    let mut filter = 0u32;
    println!("Filter {}", filter);
    input
        .iter()
        .take(14 - 1)
        .for_each(|c| filter ^= 1 << (c % 32));


    println!("Filter First After {} {:b}", filter, filter);

    input.windows(14).position(|w| {
        let first = w[0];
        let last = w[w.len() - 1];
        println!("Check {first} {last}");
        filter ^= 1 << (last % 32);
        println!("Filter Before {} {:b}", filter, filter);
        let res = filter.count_ones() == 14 as _;
        filter ^= 1 << (first % 32);
        println!("Filter After {} {:b}", filter, filter);
        res
    })
}


fn main() {
    let input = "abablmnocdefghjklmnoapqrstu";
    println!("Bytes {:?}", input.as_bytes());
    println!("{:?}", benny(input.as_bytes()));
}