use std::{
    fs::{metadata, read_dir, File, OpenOptions},
    io::{Read, Write, Result},
    path::{Path, PathBuf},
};
use rs_merkle::{
    algorithms::Sha256,
    Hasher,
};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub hash: Vec<u8>,
}

impl FileInfo {
    #[allow(dead_code)]
    pub fn new(path: PathBuf, hash: Vec<u8>) -> Self {
        Self { path, hash }
    }
}

pub fn add<P: AsRef<Path>>(path: P, root: &Path) -> Result<()> {
    let mut file_infos = Vec::new();

    let p = path.as_ref();
    if p.exists() {
        let metadata = metadata(p)?;
        if metadata.is_file() {
            let file_info = process_file(p)?;
            file_infos.push(file_info);
        } else if metadata.is_dir() {
            let entries = read_dir(p)?;
            for entry in entries {
                let entry = entry?;
                let entry_path = entry.path();
                let file_info = process_file(&entry_path)?;
                file_infos.push(file_info);
            }
        }
    }

    let hashes: Vec<<Sha256 as Hasher>::Hash> = file_infos.iter()
        .map(|file_info| {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&file_info.hash);
            arr
        })
        .collect();

    let serialized_leaves = bincode::serialize(&hashes)
        .expect("failed to serialize leaves");

    let staged_file_path = root.join(".tls/.staged");
    let mut file = OpenOptions::new().write(true).create(true).open(staged_file_path)?;
    file.write_all(&serialized_leaves)?;

    Ok(())
}

fn process_file(path: &Path) -> Result<FileInfo> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let hash = Sha256::hash(&buffer);

    Ok(FileInfo::new(path.to_path_buf(), hash.to_vec()))
}