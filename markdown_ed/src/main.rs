
mod cli_lib {
    pub(crate) mod imp_method;
    pub(crate) mod cli_struct;

    //pub(crate) use imp_method::*;
}

use crate::cli_lib::{cli_struct::Cli, imp_method::file_read};
use std::path::Path;



fn main() {

    let _par = Cli::parse_fn();
    let mut target_file = String::new();
    if let Some(_file_name) = _par.get_arg() {
        println!("Value for file name: {}",&_file_name);
        target_file.clone_from(_file_name);
    }

    match file_read(Path::new(&target_file)) {
        Ok(_value) => { 
            println!("Convert html value: \n{}",_value); 
        },
        Err(error) => {
            println!("Error: {}",error);
        }
    }
    println!("Hello, world!");
}
