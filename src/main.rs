use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;

fn main() {
    fn walk_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    walk_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }

    println!("Type part of a file name to search for:");

    let mut query = String::new();

    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");

    let query = query.trim().to_lowercase();

    let print_entry = |entry: &DirEntry| {
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        if file_name.contains(&query) {
            println!("{}", entry.path().display());
        }
    };

    let _dirs = walk_dirs(Path::new("./"), &print_entry);
}
