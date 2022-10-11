use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file(path:&str) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        // Ok(_) => Ok(s),
        Err(e) => Err(e),
        Ok(_) => Ok(s),
    }
}
fn main() {
    // println!("Hello, world!");
    let str_file = read_username_from_file("hello.txt");
 match str_file {
     Ok(s)=> println!("{}",s),
     Err(e)=> match e.kind() {
         io::ErrorKind::NotFound => println!("没文件"),
         _=>println!("mmei renj "),
     },
 }
}