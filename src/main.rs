use std::env;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::errors::*;
use carbone_sdk_rs::types::ApiJsonToken;

mod app;

const ERROR_EXIT_CODE: i32 = 1;

fn main() -> Result<(), CarboneError> {



    let cli = app::cli::Cli::new();

    let config_file_path = cli.get_path_from_option(&cli.config);
    let config = match Config::from_file(config_file_path.as_str()) { 
        Ok(config) => config,
        Err(_) => Default::default()
    };
    
    let app = app::carbone::App::new(&config, &token, &cli)?;
    app.run()?;
    
    Ok(())
}
