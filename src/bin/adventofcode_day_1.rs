use std::time::Instant;


fn filter_solution(data: &str) -> u32{
    data
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_digit(10));
            let first = match chars.next() {
                Some(num) => {num}
                None => return 0 // just return 0 if no digits are found
            };
            // create num string from the last value, when no
            let num = match chars.last() {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            };
            num.parse::<u32>().unwrap()
        })
        .sum()
}

fn find_map(data: &str) -> u32{
    data
        .lines()
        .map(|line|{
            let first = line.chars()
                .find_map(|s| s.to_digit(10))
                .unwrap_or(0);

            let last = line.chars()
                .rev()
                .find_map(|s| s.to_digit(10))
                .unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn my_solution(data: &str) -> u32{
    data
        .lines()
        .map(|line|{
            let first = line.chars()
                .flat_map(|s| s.to_digit(10))
                .next()
                .unwrap_or(0);

            let last = line.chars()
                .rev()
                .flat_map(|s| s.to_digit(10))
                .next()
                .unwrap_or(first);
            first * 10 + last
        })
        .sum()
}


fn my_solution_2(data: &str) -> u32{
    data
        .lines()
        .map(|line|{
            let mut value = String::new();
            for num in line.chars(){
                if num.is_digit(10){
                    value.push(num);
                    break
                }
            }

            for num in line.chars().rev(){
                if num.is_digit(10){
                    value.push(num);
                    break
                }
            }
            value.parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    let documents: &str = "1abc2
asada
pqr3stu8vwx
a1b2c3d4e5f

treb7uchet
";
    let now = Instant::now();
    println!("Find Map {:?},", my_solution(documents));
    let elapsed = now.elapsed();
    println!("Find Map: {:.2?}", elapsed);

    /*let now = Instant::now();
    println!("My Solution {:?},", my_solution(documents));
    let elapsed = now.elapsed();
    println!("My Solution Elapsed: {:.2?}", elapsed);*/

    /*let now = Instant::now();
    println!("Filter Solution {:?},", filter_solution(documents));
    let elapsed = now.elapsed();
    println!("Filter Solution Elapsed: {:.2?}", elapsed);*/

    /*let now = Instant::now();
    println!("[My Solution 2] {:?},", my_solution_2(documents));
    let elapsed = now.elapsed();
    println!("My Solution 2 Elapsed: {:.2?}", elapsed);*/
}