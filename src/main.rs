mod args;

use clap::Parser;
use args::RgetArgs;
use std::fs::File;
use std::io::copy;
use reqwest::Url;

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
        // Open file for writing | use full_file_name
        let mut file = File::create(&*full_file_name)?;

        // Copy response to file
        copy(&mut response, &mut file)?;

        println!("File downloaded successfully.");
    } else {
        println!("Error while downloading file.");
    }

    Ok(())
}
