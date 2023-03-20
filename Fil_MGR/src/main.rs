#[warn(unused_variables)]
use std::{error::Error};
use std::{result::Result, io::{self, Write}};
use clap::{Parser, Subcommand, Args};
use walkdir::WalkDir;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct AddArgs {
    /// Enter  from and to
    path: Vec<String>,

    /// target is directory
    #[arg(short='d', long="directory")]
    is_directory: bool,
}

#[derive(Args)]
struct AddArgsshow {
    /// Enter  path , Default current directory
    path: String,

}

#[derive(Subcommand)]
enum Commands {
    /// search file or directory
    Search(AddArgs),

    /// delete file
    Delete(AddArgs),

    /// list item of a directory
    Show(AddArgsshow),

    /// copy file
    Cyfile(AddArgs),

    /// move or rename file
    MoRfile(AddArgs),
}

fn travel_directory(file_name:&Vec<String>, is_directory:&bool) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(&file_name[0])
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_directory || e.file_type().is_dir())
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let _dir = entry.path();
        if f_name.contains(&file_name[1]) {
            println!("{}",_dir.display().to_string().replace("\\", "/"));
        }
    }

    Ok(())
}

fn cyfile_func(file_name:&Vec<String>) -> std::io::Result<()> {
    fs::copy(&file_name[0], &file_name[1])?;  // Copy from to target
    Ok(())
}

fn mvrnfile_func(file_name:&Vec<String>) -> std::io::Result<()> {
    fs::rename(&file_name[0], &file_name[1])?;
    Ok(())
}

fn delete_target(file_name:&Vec<String>,is_directory:bool)-> std::io::Result<()>{
    let mut input = String::new();
    print!("Confirm delete target: {:?} Enter (y/n): ", file_name);
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    match input {
        "y" | "Y" => {
            if !is_directory {
                for file in file_name {
                    fs::remove_file(file)?;
                }
            }
            else{
                for file in file_name {
                    fs::remove_file(file)?;
                }
            }
            println!("Deleted {:?} successfully", file_name);
        },
        "n" | "N" => println!("Cancelled delete command"),
        _ => println!("y/n only please."),
    }


    Ok(())
}

fn show_directory(file_name:&str) -> Result<(), Box<dyn Error>> {

    for entry in WalkDir::new(file_name).max_depth(1) {

        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    println!("[dir]  {}", entry.path().display().to_string().replace("\\", "/"));
                }
                else {
                    println!("[file]  {}", entry.path().display().to_string().replace("\\", "/"));
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        };   
        
    }
    
    Ok(())
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search(name) => {
            if name.path.len() == 2 {
                if let Err(error) = travel_directory(&name.path,&name.is_directory) {println!("Error message: {}",error);}
            }
            else {println!("Error: Please Enter correct format!!");}
        },
        Commands::Delete(name) => {
            if let Err(error) = delete_target(&name.path,name.is_directory) {println!("Error message: {}",error);}
        },
        Commands::Show(name) => {
            if let Err(error) = show_directory(&name.path) {println!("Error message: {}",error);}
        },
        Commands::Cyfile(name) => {
            if name.path.len() == 2 {
                if let Err(error) = cyfile_func(&name.path) {println!("Error message: {}",error);}
            }
            else {println!("Error: Please Enter correct format!!");}
        },
        Commands::MoRfile(name) => {
            if name.path.len() == 2 {
                if let Err(error) = mvrnfile_func(&name.path) {println!("Error message: {}",error);}
            }
            else {println!("Error: Please Enter correct format!!");}
        }
    }

    /*match travel_directory() {
        Ok(_) => {/*println!("OK")*/},
        Err(error) => println!("Error message: {}",error),
    }*/
    
}