mod args;

use args::RgetArgs;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = RgetArgs::parse();
    println!("{:?}", args);

    // get file name from url
    let file_name = args.url.split("/").last().unwrap();
    // make from argument full file name
    let full_file_name = args.directory + "/" + file_name;
    // Make file url
    let url = Url::parse(&*args.url)?;

    // Make GET request
    let mut response = reqwest::blocking::get(url)?;

    // Test if response is success
    if response.status().is_success() {
        // Get total file size from response headers
        let total_size = response.content_length().unwrap_or(0);

        // Create progress bar
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar());

        // Open file for writing | use full_file_name
        let mut file = File::create(&*full_file_name)?;

        // Read response in chunks and write to file with progress update
        let mut buffer = [0; 8192]; // Buffer size of 8KB
        let mut downloaded = 0;
        loop {
            let bytes_read = response.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read;
            pb.set_position(downloaded as u64);
        }

        pb.finish_with_message("File downloaded successfully.");
    } else {
        println!("Error while downloading file.");
    }

    Ok(())
}
