use reqwest::Url;
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter};
use indicatif::{ProgressBar, ProgressStyle};
use colorful::{Color, Colorful};
use log::{debug, error, info};

pub async fn download_file(url: &str, save_path: &str, pb_chars: &str, pb_style: &str) -> Result<(), Box<dyn Error>> {
    info!("Starting to download the file from: {}", url);
    // Parse URL
    let url = Url::parse(url)?;

    // Make GET request
    debug!("Sending GET request");
    let mut response = reqwest::get(url.clone()).await?;

    if response.status().is_success() {
        info!("Received successful response");
        // Get total file size from response headers
        let total_size = response.content_length().unwrap_or(0);
        debug!("Total size of the file: {}", total_size);
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(pb_style)
                .unwrap()
                .progress_chars(pb_chars), // apply parsed config
        );

        // Open file for writing
        debug!("Opening file for writing: {}", save_path);
        let file = File::create(save_path)?;
        let mut buffered_file = BufWriter::new(file);

        // Read response in chunks and write to file with progress update
        let mut downloaded = 0;
        while let Some(chunk) = response.chunk().await? {
            buffered_file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        info!("Flushing buffer to ensure all data is written to disk");
        buffered_file.flush()?; // Flush the buffer to ensure all data is written to disk

        info!("Successfully downloaded the file from: {}", url);
        Ok(())
    } else {
        error!("Error during file download, status: {}", response.status());
        let error_message = format!("Error: {}", response.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message.color(Color::Red).to_string(),
        )))
    }
}