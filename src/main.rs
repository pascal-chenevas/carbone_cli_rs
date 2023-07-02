use std::env;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::errors::*;
use carbone_sdk_rs::types::*;

mod app;

const ERROR_EXIT_CODE: i32 = 1;

fn main() -> Result<(), CarboneError> {

    let token =  match env::var("CARBONE_TOKEN") {
        Ok(v) => {
            match ApiJsonToken::new(v) {
                Ok(token) => token,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(ERROR_EXIT_CODE)
                }
            }
        },
        Err(_) => {
            println!("\nEnvironment Variable `CARBONE_TOKEN` is not set!\n");
            std::process::exit(ERROR_EXIT_CODE)
        }
    };

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