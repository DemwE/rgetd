mod args;
mod config;
mod download;

use clap::Parser;
use log::{LevelFilter, info, debug};
use config::ConfigPath;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    if args::Args::parse().debug {
        env_logger::builder().filter(None, LevelFilter::Debug).init();
        info!("Debug mode is enabled");
    }
    // Parse arguments
    let args = args::Args::parse();
    //Debug arguments
    debug!("Parsed arguments: {:#?}", args);

    let config = ConfigPath::new();
    info!("Validating config file");
    config.check();
    let parsed_config = config.read();
    //Debug config
    debug!("Parsed config: {:#?}", parsed_config);

    // Get the filename if provided, otherwise extract it from the URL
    let file_name = match args.name {
        Some(name) => name,
        None => {
            let url = Url::parse(&args.url)?;
            let url_path = Url::path_segments(&url).ok_or("Failed to parse URL")?;
            url_path.last().ok_or("Failed to extract filename from URL")?.to_string()
        }
    };
    debug!("Filename from arguments or URL: {}", file_name);

    // make from argument full file name
    let full_file_name = format!("{}/{}", args.save_directory, file_name);
    debug!("Full file name is: {}", full_file_name);

    download::download_file(&args.url, &full_file_name, &parsed_config.progress_bar_chars, &parsed_config.progress_bar_style).await?;

    Ok(())
}