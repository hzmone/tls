use std::{
    time::{SystemTime, UNIX_EPOCH},
    fs::{File, OpenOptions},
    io::{Write, Read, Result},
    path::Path,
};
use bincode;
use rs_merkle::{
    MerkleTree,
    algorithms::Sha256,
};
use serde::{Serialize, Deserialize};
use sha2::Digest;
use hex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub id: String,
    pub timestamp: u64,
    pub merkle_root: [u8; 32],
    pub parent_id: Option<String>,
}

impl Commit {
    pub fn new(merkle_root: [u8; 32], parent_id: Option<String>) -> Self {
        let id = generate_commit_id();
        let timestamp = current_timestamp();
        Self {
            id,
            timestamp,
            merkle_root,
            parent_id,
        }
    }
}

fn generate_commit_id() -> String {
    let timestamp = current_timestamp();

    let timestamp_str = timestamp.to_string();

    let mut hasher = sha2::Sha256::new();
    hasher.update(timestamp_str.as_bytes());
    let result = hasher.finalize();

    hex::encode(result)
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards apparently. In order to abide by intergalactic law I must arrest you for time travel.\n")
        .as_secs()
}

pub fn create_commit<P: AsRef<Path>>(merkle_tree: &MerkleTree<Sha256>, parent_id: Option<String>, commit_dir: P) -> Result<()> {
    let merkle_root = merkle_tree.root().unwrap_or_default();

    let commit = Commit::new(merkle_root, parent_id);

    let serialized_commit = bincode::serialize(&commit)
        .expect("failed to serialize commit");

    let commit_file_path = commit_dir.as_ref().join(format!("{}.commit", commit.id));
    let mut file = OpenOptions::new().write(true).create(true).open(commit_file_path)?;
    file.write_all(&serialized_commit)?;

    Ok(())
}

pub fn read_commit<P: AsRef<Path>>(commit_id: &str, commit_dir: P) -> Result<Commit> {
    let commit_file_path = commit_dir.as_ref().join(format!("{}.commit", commit_id));
    let mut file = File::open(commit_file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let commit: Commit = bincode::deserialize(&buffer)
        .expect("failed to deserialized commit");

    Ok(commit)
}

pub fn rollback<P: AsRef<Path>>(commit_id: &str, commit_dir: P, target_dir: P) -> Result<()> {
    let commit = read_commit(commit_id, commit_dir)?;

    let _root_hash = commit.merkle_root;

    /*  todo: 
    * 1: add logic to restore file system from the commit using the merkle tree to verify and restore their states.
    */

    Ok(())
}