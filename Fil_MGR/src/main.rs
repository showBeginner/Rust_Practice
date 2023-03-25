#[warn(unused_variables)]
use std::{error::Error};
use std::{result::Result, io::{self, Write, Read}, fs::File, path::PathBuf};
use clap::{Parser, Subcommand, Args};
use walkdir::{WalkDir, DirEntry};
use std::fs;
use zip::write::FileOptions;

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
#[derive(Args)]
struct Addzip {
    /// Enter  path , Default current directory
    path: String,

    ///zip file name
    target_name: String,


    ///if Compress file
    #[arg(short='f', long="file")]
    is_file:bool,

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
    Morfile(AddArgs),

    ///Zip file
    Ziiff(Addzip),

    ///Extract zip file
    Exxtr(Addzip),
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

fn extra_target_file(src_path:&String, ds_path:&String) -> std::result::Result<(),Box<dyn Error>> {
    let convert = PathBuf::from(&src_path);
    let target = std::fs::File::open(&convert)?;
    let mut zip = zip::ZipArchive::new(target)?;

    zip.extract(&ds_path).map_err(|e| format!("Fail to extract {:?} : {}",ds_path,e))?;


    Ok(())
}

fn compress_target_dir(input_dir:&String,target_name:&String) -> std::result::Result<(),Box<dyn Error>>{
    let zip_file = std::fs::File::create(target_name)?;
    let dir = WalkDir::new(input_dir);

    let result = zip_myfile(zip_file, input_dir,&mut dir.into_iter().filter_map(|e| e.ok()));

    match result {
        Ok(_value) => {
            println!("Compress target Successful!!!!!!");
        }
        Err(error) => {
            println!("Compress target Failed: {:?}",error);
        }
    }
    
    
    Ok(())
}

fn compress_target_file(input_dir:&String,target_name:&String) -> std::result::Result<(),Box<dyn Error>>{
    let zip_file = std::fs::File::create(target_name)?;
    let dir = WalkDir::new(input_dir);

    let convert = PathBuf::from(&input_dir);
    let _prefix = convert.parent().map_or_else(|| "/",|p| p.to_str().unwrap());
    let result = zip_myfile(zip_file, &_prefix.to_string(),&mut dir.into_iter().filter_map(|e| e.ok()));

    match result {
        Ok(_value) => {
            println!("Compress target Successful!!!!!!");
        }
        Err(error) => {
            println!("Compress target Failed: {:?}",error);
        }
    }
    
    
    Ok(())
}

fn zip_myfile(target:File,src_dir:&String,enttry:&mut dyn Iterator<Item = DirEntry>) -> Result<(),Box<dyn Error>> {
    let mut buf = Vec::new();
    let mut zip = zip::ZipWriter::new(target);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for entry in enttry{
        let path = entry.path();
        let name = path.strip_prefix(src_dir).unwrap();
        if path.is_file() {
            println!("adding file {:?} as {:?} ...",path,name);
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buf)?;
            zip.write_all(&buf)?;
            buf.clear();
        }
        else if name.as_os_str().len() != 0{
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }
    // Apply the changes you've made.
    // Dropping the `ZipWriter` will have the same effect, but may silently fail
    zip.finish()?;
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
        Commands::Morfile(name) => {
            if name.path.len() == 2 {
                if let Err(error) = mvrnfile_func(&name.path) {println!("Error message: {}",error);}
            }
            else {println!("Error: Please Enter correct format!!");}
        },
        Commands::Ziiff(name) => {
            if !name.is_file {
                if let Err(error) = compress_target_dir(&name.path, &name.target_name) {
                    println!("Error message: {}",error);
                }
            }
            else {
                if let Err(error) = compress_target_file(&name.path, &name.target_name) {
                    println!("Error message: {}",error);
                }
            }
            
        },
        Commands::Exxtr(name) => {
            if let Err(error) = extra_target_file(&name.path,&name.target_name) 
            {println!("Error message: {}",error);}
        }
    }

    /*match travel_directory() {
        Ok(_) => {/*println!("OK")*/},
        Err(error) => println!("Error message: {}",error),
    }*/
    
}