mod args;

use args::RgetArgs;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use std::fs::File;
use std::io::{Read, Write, BufWriter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create stdout
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Parse arguments
    let args = RgetArgs::parse();


    // get file name from url
    // check if argument -n or --name is used
    let file_name = if args.name.is_some() {
        args.name.unwrap()
    } else {
        args.url.split("/").last().unwrap().to_string()
    };

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
        pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} | {binary_bytes_per_sec} | eta {eta} ")
            .unwrap()
            .progress_chars("#>-"));

        // Open file for writing | use full_file_name
        let file = File::create(&*full_file_name)?;
        let mut buffered_file = BufWriter::new(file);

        // Read response in chunks and write to file with progress update
        let mut buffer = [0; 65536]; // Buffer size of 64KB
        let mut downloaded = 0;
        loop {
            let bytes_read = response.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            buffered_file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read;
            pb.set_position(downloaded as u64);
        }

        buffered_file.flush()?; // Flush the buffer to ensure all data is written to disk

        pb.finish_with_message("File downloaded successfully.");
    } else {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
        println!("Error while downloading file.");
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
    }

    Ok(())
}
