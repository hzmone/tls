use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::{Read, Write, Result},
    path::Path,
};
use rs_merkle::{
    algorithms::Sha256,
    MerkleTree,
    Hasher,
};
use serde::{Serialize, Deserialize};
use bincode;

const STAGED_PATH: &str = ".tls/.staged";
const COMMIT_DIR: &str = ".tls/commits";
const LATEST_COMMIT_PATH: &str = ".tls/latest_commit";

#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub id: <Sha256 as rs_merkle::Hasher>::Hash,
    pub parent: Option<<Sha256 as rs_merkle::Hasher>::Hash>,
    pub message: String,  // Store the commit message
}

impl Commit {
    pub fn new(message: String, parent: Option<<Sha256 as rs_merkle::Hasher>::Hash>) -> Self {
        let id = Sha256::hash(message.as_bytes());  // Simplified ID generation for the commit
        Self { id, parent, message }
    }
}

pub fn commit(message: String) -> Result<()> {
    // Ensure the commit directory exists
    create_dir_all(COMMIT_DIR)?;

    // Read the staged Merkle tree leaves
    let mut merkle_tree = read_merkle_tree(STAGED_PATH)?;

    // Commit the Merkle tree (using rs_merkle's commit logic)
    merkle_tree.commit();

    // Determine the parent commit's ID
    let parent_commit_id = if Path::new(LATEST_COMMIT_PATH).exists() {
        let parent_commit = read_commit(LATEST_COMMIT_PATH)?;
        Some(parent_commit.id)
    } else {
        None
    };

    // Create a new commit with the provided message
    let new_commit = Commit::new(message, parent_commit_id);

    // Store the commit metadata (including the commit message)
    store_commit_metadata(&new_commit)?;

    Ok(())
}

fn read_merkle_tree<P: AsRef<Path>>(path: P) -> Result<MerkleTree<Sha256>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let hashes: Vec<<Sha256 as Hasher>::Hash> = bincode::deserialize(&buffer)
        .expect("failed to deserialize merkle leaves");

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&hashes);

    Ok(merkle_tree)
}

fn read_commit<P: AsRef<Path>>(path: P) -> Result<Commit> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let commit: Commit = bincode::deserialize(&buffer)
        .expect("failed to deserialize commit");

    Ok(commit)
}

fn store_commit_metadata(commit: &Commit) -> Result<()> {
    // Serialize and store the commit metadata
    let serialized_commit = bincode::serialize(commit)
        .expect("failed to serialize commit");

    let commit_file = format!("{}/{}.commit", COMMIT_DIR, hex::encode(commit.id));
    let commit_path = Path::new(&commit_file);
    let mut file = OpenOptions::new().write(true).create(true).open(commit_path)?;
    file.write_all(&serialized_commit)?;

    // Update the latest commit file
    let mut latest_commit_file = OpenOptions::new().write(true).create(true).truncate(true).open(LATEST_COMMIT_PATH)?;
    latest_commit_file.write_all(commit.id.as_slice())?;

    Ok(())
}
