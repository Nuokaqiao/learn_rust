use std::path::Path;
fn main() {
    let path = Path::new("./src");
    println!("{}", path.is_dir());
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
            // println!("{:?}", entry.file_type());
        }
    }
}
