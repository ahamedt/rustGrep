use std::env;
use std::fs;

fn oldmain() {
    // Issue 1: Too many responsibilties
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("In file {file_path}");
    // Issue 2: Contents provides the main logic for the program. Keep config variables like 
    // query and file_path in a struct?
    
    // Issue 3: The expect failure would print the same thing for any kind of file reading failure.
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
