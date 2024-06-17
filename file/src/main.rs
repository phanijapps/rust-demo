use std::fs;

// Read content file from Filesystem
fn main() {
    let file_content = fs::read_to_string("f:/test.txt").expect("Should have been able to read the file");

    println!("With text:\n{file_content}");
}
