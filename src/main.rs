
use clap::Parser;

use std::env;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::template::*;
use carbone_sdk_rs::render::RenderOptions;
use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::errors::*;
use carbone_sdk_rs::types::*;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

/// Simple CLI-App to generate a report using the API of Carbone (http://carbone.io)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path of the config file
    #[arg(short, long, required = false)]
    config_file_path: String,
    
    /// json data to be rendered
    #[arg(short, long, required = false)]
    json_data: String,
   
    /// template file
    #[arg(short, long)]
    template: String,

    /// output file for the generated report
    #[arg(short, long, required = false)]
    output: String,
}

fn main() -> Result<(), CarboneError> {
    
    let args = Args::parse();

     let token =  match env::var("CARBONE_TOKEN") {
             Ok(v) => v,
             Err(e) => panic!("{}", e.to_string())
     };
 
    let config = &Config::from_file(args.config_file_path.as_str())?;
 
    let api_token = &ApiJsonToken::new(token)?;

    let json_data = fs::read_to_string(args.json_data)?;
 
    let render_options = RenderOptions::new(json_data)?;

    let carbone_sdk = Carbone::new(config, api_token)?;

    let template_file = TemplateFile::new(args.template)?;
    
    let report_content = carbone_sdk.generate_report_with_file(&template_file, render_options, "")?;

    let mut file = File::create(&args.output)?;
    file.write_all(&report_content)?;

    let metadata = fs::metadata(&args.output)?;

    println!("");
    println!("file {} written {} byte(s)", &args.output, metadata.len());
    println!("");

    Ok(())
}