use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Result, Write};
use std::os::unix::fs::FileExt;
use chrono::Local;


fn test_file_vector_buffer() -> Result<()> {
   let test_path = "assets/test.txt";
   let mut file = File::open(test_path)?;
   let mut reader = BufReader::new(file);
   let mut vec = Vec::with_capacity(50);

   reader.read_to_end(&mut vec)?;
   println!("Raw bytes: {:?}", vec);
   
   // Using std::str::from_utf8 - doesn't consume the vector and is simpler
   match std::str::from_utf8(&vec) {
       Ok(s) => println!("As string: {}", s),
       Err(e) => println!("Not valid UTF-8: {}", e),
   }
   
   let mut file = File::create("assets/vec_output.txt")?;
   let mut writer = BufWriter::new(file);
   writer.write_all(&vec)?;
   writer.flush()?;

   Ok(())
}

fn test_file_str_buffer() -> Result<()> {
    let test_path = "assets/test.txt";
    let mut file = File::open(test_path)?;
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    // reader.read_until(byte, buf)
    reader.read_to_string(&mut str)?;
    println!("{}", str);


    let mut file = File::create("assets/write_and_update.txt")?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}\n", str)?;
    for x in 0..5{
        writeln!(writer, "the loopline {}", x)?;
    }
    writer.flush()?;
    Ok(())
}

fn test_generate_new_file() -> Result<()> {
    let fname = "./assets/new-1.jpeg";
    let mut file = File::open(fname)?;
    let mut reader = BufReader::new(file);
    
    let ext = &fname[fname.rfind(".").unwrap()..];
    let new_fname = format!(
        "./assets/generate_{}{}", Local::now().timestamp(), ext
    );
    let mut file_writer = File::create(new_fname)?;
    let mut buff_writer = BufWriter::new(file_writer);
    let mut buffer = [0u8; 100];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        buff_writer.write_all(&buffer[..read])?;
    }
    buff_writer.flush()?;
    Ok(())

}


fn main() -> Result<()> {
    let test_path = "assets/test.txt";
    let mut file = File::open(test_path)?;
    let mut text = String::new();
    // println!("File opened: {:?}", file);
    file.read_to_string(&mut text)?;
    println!("{}", text);
    //let reader = BufReader::new(file);
    //let mut lines = reader.lines();

    // Example of creating and opening a file with read/write permissions
    let mut custom_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("assets/new_test.txt")?;
    
    println!("Writing to file...");
    println!("{}", custom_file.write(b"first line\n")?);
    println!("{:?}", custom_file.write_all(b"10 offset\n")?);
    custom_file.write_all_at(b"5 offset\n", 16)?;

    // example of reading a file with a buffer and string
    println!("Testing file string buffer...");
    test_file_str_buffer()?;
    
    // example of reading a file with a vector buffer
    println!("Testing file vector buffer...");
    test_file_vector_buffer()?;
    //test make copy of image file
    test_generate_new_file()?;
    Ok(())
}
