//! Compare rust performance using bit and without

// use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

/// Find using array
pub fn find_unique_numbers(numbers: &[u32; 25]) -> [u32; 25]{
    let mut count = 0;
    let mut xnums: [u32; 25] = [0;25];
    // println!("Nums {:?}", xnums);
    for number in numbers{
        if !xnums.contains(&number){
            xnums[count] = *number;
            //println!("Nums 2 {:?}", xnums);
            count += 1;
        }
    }
    //println!("Nums last {:?}", xnums);
    xnums
}

/// Find using bit by replace contains
///
/// # Examples
///
/// ```
/// let f = find_unique_number_bit(&[1..26]);
/// ```
pub fn find_unique_number_bit(numbers: &[u32;25]) -> [u32; 25]{
    /// Testing make a doc
    /// another description
    let mut bitset: u32 = 0;
    let mut count = 0;
    let mut xnums: [u32; 25] = [0;25];

    for number in numbers{
        let mask: u32 = 1 << (number - 1);
        let and = bitset & mask;
        println!("Mask {} {:b} - Num {}", mask, mask, number);
        println!("bitset {} {:b} - result AND {}", bitset, bitset, and);
        bitset |= mask;
        println!("Binary - Mask {:b}, bitset {} {:b}", mask, bitset, bitset);
        if and == 0 {
            xnums[count] = *number;
            count += 1;
        }
    }
    xnums
}

/// Find and return bitset
fn find_unique_numbers_bitset(numbers: &[u32;25]) -> u32 {
    let mut bitset: u32 = 0;
    for number in numbers {
        let mask: u32 = 1 << (number - 1);
        bitset |= mask;
    }
    bitset
}

fn find_unique_numbers_bitset_2(numbers: &[u32;25]) -> u32 {
    let mut bitset: u32 = 0;
    for number in numbers {
        let mask: u32 = 1 << (number - 1);
        bitset |= mask;
    }
    bitset_to_array_2(bitset)
}

fn bitset_to_array_2(bitset: u32) -> u32 {
    let mut x = 0;
    for i in 0..32 {
        println!("{} {} {}", (1 << i), bitset, bitset & (1 << i));
        if (bitset & (1 << i)) != 0 {
            x += i + 1;
        }
    }
    x
}

fn bitset_to_array(bitset: u32) -> Vec<u32> {
    let mut array = Vec::new();
    for i in 0..32 {
        println!("{} {} {}", (1 << i), bitset, bitset & (1 << i));
        if (bitset & (1 << i)) != 0 {
            array.push(i + 1);
        }
    }
    array
}


fn main() {
    const nums: [u32; 25] = [
        1, 2, 3, 4, 5, 6, 7, 24, 10, 11, 12, 3, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 1, 2
    ];
    println!("nums {:?}", nums);
    println!("Bit {:b} {:b}", 1, 8);
    println!("same nums {:?}", find_unique_numbers(&nums));
    println!("bit same nums {:?}", find_unique_number_bit(&nums));
    let bitset = find_unique_numbers_bitset(&nums);
    println!("bitset same nums {:?}", bitset);
    println!("bitset to array {:?}", bitset_to_array(bitset));
}

/*fn criterion_benchmark(c: &mut Criterion) {
    const nums: [u32; 25] = [
        1, 2, 3, 4, 5, 6, 7, 24, 10, 11, 12, 3, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 1, 2
    ];
    c.bench_with_input(
        BenchmarkId::new("find_unique_numbers", "numbers"), &nums,
        |b, i| b.iter(|| find_unique_numbers(i))
    );

    c.bench_with_input(
        BenchmarkId::new("find_unique_number_bit", "numbers"), &nums,
        |b, i| b.iter(|| find_unique_number_bit(i))
    );

    c.bench_with_input(
        BenchmarkId::new("find_unique_number_bitset", "numbers"), &nums,
        |b, i| b.iter(|| find_unique_numbers_bitset(i))
    );

    c.bench_with_input(
        BenchmarkId::new("find_unique_number_bitset_2", "numbers"), &nums,
        |b, i| b.iter(|| find_unique_numbers_bitset_2(i))
    );
}

criterion_group!(compare, criterion_benchmark);
criterion_main!(compare);
*/


/*
Benchmark result

Benchmarking find_unique_numbers/numbers: Collecting 100 samples in estimated 5.0000 sfind_unique_numbers/numbers
                        time:   [149.57 ns 151.85 ns 154.54 ns]
                        change: [+3.1577% +4.8486% +6.7816%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

Benchmarking find_unique_number_bit/numbers: Warming up for 3.0000 s


Benchmarking find_unique_number_bit/numbers: Collecting 100 samples in estimated 5.0000 s (241M iterations)
find_unique_number_bit/numbers
                        time:   [19.585 ns 19.794 ns 20.019 ns]
                        change: [-7.7166% -5.8675% -4.1693%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

Benchmarking find_unique_number_bitset/numbers: Warming up for 3.0000 s

Benchmarking find_unique_number_bitset/numbers: Collecting 100 samples in estimated 5.find_unique_number_bitset/numbers
                        time:   [5.2859 ns 5.3289 ns 5.3767 ns]
                        change: [-3.8936% -2.7518% -0.9422%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

Benchmarking find_unique_number_bitset_2/numbers: Warming up for 3.0000 s

Benchmarking find_unique_number_bitset_2/numbers: Collecting 100 samples in estimated find_unique_number_bitset_2/numbers
                        time:   [11.070 ns 11.170 ns 11.280 ns]
                        change: [-7.5188% -5.7715% -4.3114%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

 */