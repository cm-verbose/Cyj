use std::{fs, panic, path::Path};

pub fn read() -> String {
    /* For testing only */
    let file_path: &Path = Path::new("src/test.cyj");

    if !file_path.exists() {
        /* Custom error message */
        panic::set_hook(Box::new(|_| print!("Cannot find file ")));
        panic!();
    };

    return fs::read_to_string(file_path).unwrap();
}
