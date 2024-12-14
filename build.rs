use regex::Regex;
use std::fs::File;
use std::io::{copy, Read};
use std::path::Path;
use std::fmt::format;

fn download_cacert(url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dest_path.exists() {
        let mut response = reqwest::blocking::get(url)?;
        let mut dest_file = File::create(dest_path)?;
        copy(&mut response, &mut dest_file)?;
    }
    Ok(())
}

fn extract_date_from_cacert(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"Certificate data from Mozilla as of: (.*)")?;
    let date = re
        .captures(&contents)
        .and_then(|cap| cap.get(1))
        .map_or("Unknown date".to_string(), |m| m.as_str().to_string());

    Ok(date)
}

fn main() {
    let url = "https://curl.se/ca/cacert.pem";
    let out_dir = "./caroot/";
    let dest_path = Path::new(&out_dir).join("cacert.pem");

    download_cacert(url, &dest_path).expect("Failed to download cacert.pem");

    let date =
        extract_date_from_cacert(&dest_path).expect("Failed to extract date from cacert.pem");

    let help_ca_root: String = format(format_args!("If not set, the bundled cacert.pem from Mozilla ({}) will be used." , date));

    println!("cargo:rustc-env=CACERT_DATE={}", date);
    println!("cargo:rustc-env=HELP_CA_ROOT={}", help_ca_root);
}
