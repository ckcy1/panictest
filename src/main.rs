use std::io;
use std::io::Read;
use std::fs::File;
fn main() {
    // println!("Hello, world!");
    fn read_username_from_file()->Result<String,io::Error>{
        let f=File::open("hello.txt");
        let mut f = match f {
            Ok(file)=>file,
            Err(e)=>{
                return Err(e);
            }
            let mut s = string::new();
            match f.read_to_string(&mut s) {
                Ok(_)=>{Ok(s),
                    Err(e)=>Err(e),
        }
    }
        }
            