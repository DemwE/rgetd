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


    // get file name from url
    // check if argument -n or --name is used
    let file_name = if args.name.is_some() {
        args.name.unwrap()
    } else {
        args.url.split("/").last().unwrap().to_string()
    };

    println!("file name: {}", file_name);

    // make from argument full file name
    let full_file_name = args.save_directory + "/" + &*file_name;
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
        pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

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
