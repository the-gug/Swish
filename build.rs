use std::fs::File;
use std::io::copy;
use std::path::Path;


fn main() {
    let url = "https://curl.se/ca/cacert.pem";
    let out_dir = "./caroot/";
    let dest_path = Path::new(&out_dir).join("cacert.pem");

    if !dest_path.exists() {
        let mut response = reqwest::blocking::get(url).expect("Failed to download cacert.pem");
        let mut dest_file = File::create(&dest_path).expect("Failed to create cacert.pem file");
        copy(&mut response, &mut dest_file).expect("Failed to copy content to cacert.pem file");
    }
}