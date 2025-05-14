

use std::env;


fn main() {


   let args: Vec<String> = env::args().collect();
    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
}


