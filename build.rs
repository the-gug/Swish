use regex::Regex;
use std::fs::File;
use std::io::{copy, Read};
use std::path::Path;
use std::fmt::format;

/// Downloads the cacert.pem file from the provided URL and saves it to the specified destination path.
///
/// # Arguments
///
/// * `url`: The URL of the cacert.pem file to download.
/// * `dest_path`: The destination path where the cacert.pem file will be saved.
///
/// # Returns
///
/// * `Result`: An empty result if the operation is successful, or an error if it fails.
fn download_cacert(url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dest_path.exists() {
        let mut response = reqwest::blocking::get(url)?;
        let mut dest_file = File::create(dest_path)?;
        copy(&mut response, &mut dest_file)?;
    }
    Ok(())
}

/// Extracts the date from the cacert.pem file at the specified file path.
///
/// # Arguments
///
/// * `file_path`: The path to the cacert.pem file.
///
/// # Returns
///
/// * `Result`: The extracted date as a string if successful, or an error if it fails.
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
    // Specify the URL of the cacert.pem file and the output directory
    let url = "https://curl.se/ca/cacert.pem";
    let out_dir = "./caroot/";
    let dest_path = Path::new(&out_dir).join("cacert.pem");

    // Download the cacert.pem file
    download_cacert(url, &dest_path).expect("Failed to download cacert.pem");

    // Extract the date from the downloaded cacert.pem file
    let date =
        extract_date_from_cacert(&dest_path).expect("Failed to extract date from cacert.pem");

    // Format the help message with the extracted date
    let help_ca_root: String = format(format_args!("If not set, the bundled cacert.pem from Mozilla ({}) will be used." , date));

    // Pass the date and help message to the build process
    println!("cargo:rustc-env=CACERT_DATE={}", date);
    println!("cargo:rustc-env=HELP_CA_ROOT={}", help_ca_root);
}
