// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:45:53 to 1:55:45

// rust doesn't have convenient error handling like in python
// instead, almost every error is recoverable through "Result" (.expect() previously)
// if the error can not be recoverable, use macrl pacnic!

#![allow(non_snake_case)]
use std::io::{Write, BufRead, BufReader};
use std::fs::File;

fn main() {
    let path: &str = "test_lines.txt";
    // create a file and edit it
    // the file will automatically close at the end of its scope
    create_edit_txt(path);
    // load the file
    load_print_txt(path);
}

fn create_edit_txt(path: &str){
    // when panic! is called, the memory cleaned and the programe quit itself
    // The "Result" struct will return either 
        // Ok(T) when function executed as expected
        // or
    // Err(E) when function can not be executed
    let output= File::create(path);
    let mut output = match output{
        Ok(file) => file,
        Err(error) => panic!("Cant create a file: {:?}",error)
    };
    write!(output, "First line \nSecond line \nThird Line.").expect(
            "Failed to edit the parsed file");
}

fn load_print_txt(path: &str){
    // .unwrap() will get strait to Ok, instead of returning struct "Result" 
    let input: File = File::open(path).unwrap();
    let buffered: BufReader<File> = BufReader::new(input);

    for line in buffered.lines(){
        println!("{}",line.unwrap());
    };
}