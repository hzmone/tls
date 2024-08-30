use std::{
    path::Path,
    fs::{
        create_dir_all,
        metadata,
        File,
        read_dir,
    },
    io::{
        Result,
        ErrorKind,
        Error,
    }
};

pub fn add(p: &Path, root: &Path) -> Result<()> {
    let tls = Path::new(".tls");
    if !tls.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "not initialized"));
    }

    if p.exists() {
        let metadata = metadata(p)?;

        let relative_path = p.strip_prefix(root).unwrap();
        let mut tracking_path = tls.join(relative_path);

        if metadata.is_file() {
            if let Some(_) = tracking_path.extension() {
                let root_name = tracking_path.file_stem().unwrap_or_default();
                tracking_path = tls.join(relative_path.with_file_name(format!("{}{}", root_name.to_string_lossy(), ".track")));
            } else {
                tracking_path.set_extension("track");
            }

            File::create(&tracking_path)?;
            println!("tracking {}", relative_path.display());
        } else if metadata.is_dir() {
            let tls_sub = tls.join(relative_path);
            create_dir_all(&tls_sub)?;

            for entry in read_dir(p)? {
                let entry = entry?;
                let path = entry.path();
                add(&path, root)?;
            }
        }
    }

    Ok(())
}