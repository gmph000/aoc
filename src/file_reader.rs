use std::fs;

pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path);
    if contents.is_err() {
        panic!("Could not find file {}", file_path);
    }
    let contents = contents.ok().unwrap();
    return contents;
}
