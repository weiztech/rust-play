#[derive(Debug, Clone, Copy)]
struct User<'a> {
    name: &'a str,
    age: u32,
}


fn get_longest_length<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a
    }
    b
}

fn get_user_longest_length<'a>(user_a: &'a User, user_b: &'a User) -> &'a User<'a> {
    if user_a.name.len() > user_b.name.len() {
        return user_a
    }

    user_b
}


fn get_user_longest_length_owned<'a>(user_a: &'a User, user_b: &'a User) -> User<'a> {
    if user_a.name.len() > user_b.name.len() {
        return *user_a
    }

    *user_b
}

#[derive(Debug)]
enum Data<'a> {
    Number(i32),
    Text(&'a str), // This variant holds a reference, so the enum needs a lifetime parameter
}

// Function that might return a Data enum containing a reference
fn process_data<'a>(input: &'a str, num: i32) -> Data<'a> {
    if input.len() > 5 {
        Data::Text(input)
    } else {
        Data::Number(num) // Note: num is copied, no lifetime issues here
    }
}



fn main() {
    let a = String::from("hello 123");
    let b = "World War";
    let longest = get_longest_length(&a, b);
    println!("{}", longest);

    let user = User {
        name: b,
        age: 100,
    };

    let user_b = User {
        name: "Ragna",
        age: 150,
    };
    
    let user_longest_name = get_user_longest_length(&user, &user_b);
    println!("Longest name:{:?}", user_longest_name);

    let user_longest_name_owned = get_user_longest_length_owned(&user, &user_b);
    println!("Longest owned name:{:?}", user_longest_name_owned);
    
    let data = process_data("Hello12345", 10);
    println!("{:?}", data);

}


