use clap::{command, Parser};

#[derive(Debug, Default, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(
help_template = "{name} {version} {author-section} {about-with-newline} \n {all-args}"
)]
pub struct RgetArgs {
    // Get the URL
    pub url: String,
    // Save directory for the file where default is current directory
    #[clap(default_value = ".")]
    pub directory: String,
    // Save file name when argument in -n or --name is used
    #[clap(short, long)]
    pub name: Option<String>,
}