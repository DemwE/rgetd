mod args;
mod config;

use args::RgetArgs;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use colorful::Colorful;
use colorful::Color;
use log::{LevelFilter, info, debug, error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    if args::RgetArgs::parse().debug {
        env_logger::builder().filter(None, LevelFilter::Debug).init();
        info!("Debug mode is enabled");
    }
    // Parse arguments
    let args = RgetArgs::parse();
    // Debug arguments
    debug!("Parsing input arguments: {:#?}", args);

    let conf = config::main();
    debug!("Configuration loaded: {:?}", conf);

    // get file name from url
    // check if argument -n or --name is used
    let file_name = if args.name.is_some() {
        args.name.unwrap()
    } else {
        args.url.split("/").last().unwrap().to_string()
    };
    debug!("File name derived from arguments: {}", file_name);

    // make from argument full file name
    let full_file_name = args.save_directory + "/" + &*file_name;
    debug!("Complete file name with directory: {}", full_file_name);

    // Make file url
    let url = Url::parse(&*args.url)?;
    debug!("URL parsed: {}", url);

    // Make GET request
    info!("About to make GET request");
    let mut response = reqwest::blocking::get(url)?;

    // Test if response is success
    if response.status().is_success() {
        info!("GET request response was successful");
        // Get total file size from response headers
        let total_size = response.content_length().unwrap_or(0);
        debug!("Content length determined from headers: {}", total_size);

        // Create progress bar
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::with_template(conf["progress_bar_style"].as_str().unwrap())
            .unwrap()
            .progress_chars(conf["progress_bar_chars"].as_str().unwrap()));
        info!("Progress bar initialized");

        // Open file for writing | use full_file_name
        debug!("Opening file for writing: {}", full_file_name);
        let file = File::create(&*full_file_name)?;
        let mut buffered_file = BufWriter::new(file);

        // Read response in chunks and write to file with progress update
        let mut buffer = [0; 65536]; // Buffer size of 64KB
        let mut downloaded = 0;
        info!("Begin reading response in chunks and writing to file");
        loop {
            let bytes_read = response.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            buffered_file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read;
            pb.set_position(downloaded as u64);
        }

        info!("Flushing buffer to ensure all data is written to disk");
        buffered_file.flush()?; // Flush the buffer to ensure all data is written to disk
        debug!("Buffer flushed and all data written to disk");

        pb.finish_with_message("File downloaded successfully.");
        debug!("File downloaded successfully, task complete");
    } else {
        println!("{}", "Error while downloading file.".color(Color::Red));
        error!("Error while downloading file");
    }

    Ok(())
}