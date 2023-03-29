
mod cli_lib {
    pub(crate) mod imp_method;
    pub(crate) mod cli_struct;

    //pub(crate) use imp_method::*;
}

use crate::cli_lib::{cli_struct::Cli, imp_method::file_read};
use std::path::Path;



fn main() ->std::io::Result<()>{

    let _par = Cli::parse_fn();

    if let Some(_file_name) = _par.get_arg() {
        println!("Value for file name: {}",_file_name);
    }

    file_read(Path::new("./README.md"))?;
    println!("Hello, world!");
    Ok(())
}
