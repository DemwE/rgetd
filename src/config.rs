use std::fs;
use directories::BaseDirs;
use toml::Value;


pub fn main() -> Value {
    // Config directory
    let config_dir = BaseDirs::new().unwrap().config_dir().to_str().unwrap().to_string();
    let config_file_path = format!("{}/rget/config.toml", config_dir);

    let default_config = "\
    progress_bar_chars = '#>-'\n\
    progress_bar_style = '[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} | {binary_bytes_per_sec} | eta {eta} '";

    // Create config file if it doesn't exist
    let config_file = std::fs::File::open(&*config_file_path);
    if config_file.is_err() {
        fs::create_dir_all(format!("{}/rget", config_dir)).unwrap();
        fs::File::create(&*config_file_path).unwrap();
        // Add default config to file
        fs::write(config_file_path.clone(), default_config).unwrap();
    }

    // Read config file
    let conf_raw =  fs::read_to_string(config_file_path.clone()).unwrap();

    // Parse config file
    let conf: toml::Value = toml::from_str(&*conf_raw).unwrap();

    return conf;
}