fn get_age(age: &str) -> Result<i8, &'static str>{
    match age.parse::<i8>() {
        Ok(parsed_age) => Ok(parsed_age),
        Err(_) => Err("Invalid age"),
    }
}

fn greet() -> Option<String>{
    Some(String::from("Hello"))
}

fn get_greet() -> Option<String>{
    Some(greet()?)
}


fn greet2(text: &str) -> &str {
    if text.to_lowercase() == "hello"{
        return "Hello World"
    }

    return  "Hi There"
}


fn get_name() -> Result<String, &'static str>{
    let name = get_greet().ok_or("yay")?;
    let age = get_age("12")?;
    Ok(name + " Asada - Age: " + &age.to_string())
}


fn main() {
    println!("{:?}", get_name());
    println!("Greet2 {}", greet2("Hello"));
    println!("Greet3 {}", greet2("Hello Greet 3"))
}