
use clap::Parser;

use std::env;

use std::ops::Deref;
use std::path::PathBuf;

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
struct Cli {
    /// a configuration which contains the api url, timeout and api version
    #[arg(short, long, value_name = "FILE", required = true)]
    config: Option<PathBuf>,
    
    /// json data to be rendered
    #[arg(short, long, value_name = "FILE")]
    json: Option<PathBuf>,
   
    /// template file
    #[arg(short, long, value_name = "FILE")]
    template: Option<PathBuf>,

    /// output file for the generated report
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

     /// delete a template with the given template_id
     #[arg(short, long, required = false)]
     delete_template: Option<String>,
}

fn generate_template(carbone_sdk: Carbone, template_file_path: String, json_data: String, output: &str) -> Result<(), CarboneError> {
    
    if  !template_file_path.is_empty() && !json_data.is_empty() && !output.is_empty(){

        let render_options = RenderOptions::new(json_data)?;
        let template_file = TemplateFile::new(template_file_path)?;

        let report_content = carbone_sdk.generate_report_with_file(&template_file, render_options, "")?;

        let mut file = File::create(output)?;
        file.write_all(&report_content)?;

        let metadata = fs::metadata(output)?;

        println!("");
        println!("file {} written - {} byte(s)", &output, metadata.len());
        println!("");
    }
    Ok(())

}

fn delete_template(config: &Config, api_token: &ApiJsonToken, template_id: TemplateId) -> Result<(), CarboneError> {
   
    let template = Template::new(config, api_token);
    let is_deleted = template.delete(template_id.clone())?;
    let template_id = template_id.as_str();

    println!("");
    if is_deleted {
        println!("template_id {} deleted", template_id)
    } else {
        println!("template_id {} deleted", template_id)
    }
    
    Ok(())
}

fn main() -> Result<(), CarboneError> {
    
    let cli = Cli::parse();

    let token =  match env::var("CARBONE_TOKEN") {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string())
    };

    let mut config = Default::default();

    if let Some(config_path) = cli.config.as_deref() {
        if let Some(path) = config_path.to_str() {
            config = Config::from_file(path)?;
        }
    }

    let mut json_data = String::from("");
    if let Some(json_path) = cli.json.as_deref() {
        if let Some(path) = json_path.to_str() {
            json_data = fs::read_to_string(path)?;
        }
    }

    let mut template_file_path = "".to_string();
    if let Some(template_path) = cli.template.as_deref() {
        if let Some(path) = template_path.to_str() {
            template_file_path = path.to_string();
        }
    }

    let mut output = "";
    if let Some(o) = cli.output.as_deref() {
        output = o;
    }

    let mut template_id = "";
    if let Some(t_id) = cli.delete_template.as_deref() {
        template_id = t_id;
    }

    let template_id = TemplateId::new(template_id.to_string())?;

    let api_token = &ApiJsonToken::new(token)?;

    let carbone_sdk = Carbone::new(&config, api_token)?;

    generate_template(carbone_sdk, template_file_path, json_data, output)?;

    delete_template(&config, &api_token, template_id)?;

    Ok(())
}