use std::fs::{
    File,
    create_dir,
};
use std::io::{
    ErrorKind,
    Result,
};

pub fn init() -> Result<()> {
    match create_dir(".tls") {
        Ok(_) => println!("init complete"),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => eprintln!("already initialized"),
            ErrorKind::PermissionDenied => eprintln!("permission denied"),
            _ => eprintln!("unknown error occurred: {}", e)
        }
    }
    File::create(".tls/.staged").unwrap();

    Ok(())
}