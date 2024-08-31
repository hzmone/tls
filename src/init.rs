use std::fs::{File, create_dir};
use std::io::Result;

pub fn init() -> Result<()> {
    let _ = create_dir(".tls");
    init_staged_file()
}

fn init_staged_file() -> Result<()> {
    let _ = File::create(".tls/.staged");
    init_commit_dir()
}

fn init_commit_dir() -> Result<()> {
    create_dir(".tls/commits")
}