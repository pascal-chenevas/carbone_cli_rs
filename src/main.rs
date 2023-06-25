
use clap::Parser;
use serde_json::json;

use std::env;

use bytes::Bytes;

use std::path::PathBuf;

use serde::{Serialize,Deserialize};

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::template::*;
use carbone_sdk_rs::render::RenderOptions;
use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::errors::*;
use carbone_sdk_rs::types::*;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct UploadResult {
    file: String,
    uploaded: bool,
    template_id: Option<String>,
    error: Option<String>
}

impl UploadResult {

    fn new(file: String, uploaded: bool, template_id: Option<String>, error: Option<String>) -> Self {
        Self {
            file,
            uploaded,
            template_id,
            error
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// a configuration which contains the api url, timeout and api version
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    
    /// json data to be rendered
    #[arg(short, long, value_name = "FILE")]
    json: Option<PathBuf>,
   
    /// template file
    #[arg(short, long, value_name = "FILE")]
    template: Option<PathBuf>,

    /// output file for the generated report
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// remove a template with the given template_id
    #[arg(short, long, required = false, value_name = "TEMPLATE_ID")]
    remove_template: Option<String>,

    /// update a template 
    #[arg(short, long, required = false)]
    update: bool,

    /// download a template 
    #[arg(short, long, required = false, value_name = "TEMPLATE_ID")]
    download_template: Option<String>,
}

fn generate_template(carbone_sdk: Carbone, template_file_path: &String, json_data: String, output: &str) -> Result<(), CarboneError> {
    
    let render_options = RenderOptions::new(json_data)?;
        let template_file = TemplateFile::new(template_file_path.to_owned())?;

        let report_content = carbone_sdk.generate_report_with_file(&template_file, render_options, "")?;

        write_file(&report_content, output)?;

    Ok(())
}

fn upload_template(config: &Config, api_token: &ApiJsonToken, template_file_path: &String) -> Result<(), CarboneError> {
   
    let template = Template::new(config, api_token);
    let upload_result = match TemplateFile::new(template_file_path.to_owned()) {
        Ok(tf) => {
            let result = template.upload(&tf, "".to_string());
            let upload_result = match result {
                Ok(id) => UploadResult::new(template_file_path.to_owned(), true, Some(id.as_str().to_string()), None),
                Err(e) => UploadResult::new(template_file_path.to_owned(), false, None, Some(e.to_string()))
            };
            upload_result
        },
        Err(e) =>  UploadResult::new(template_file_path.to_owned(), false, None, Some(e.to_string()))
    };

    let json = json!(upload_result);
    
    println!("{:#}", json);

    Ok(())
}

fn download_template(config: &Config, api_token: &ApiJsonToken, template_id: TemplateId, output: &str) -> Result<(), CarboneError> {
   
    let template = Template::new(config, api_token);
    let content = template.download(template_id)?;

    write_file(&content, output)?;
    
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

fn write_file(content: &Bytes, output: &str) -> Result<(), CarboneError> {

    let mut file = File::create(output)?;
    file.write_all(content)?;

    let metadata = fs::metadata(output)?;

    println!("");
    println!("file {} written - {} byte(s)", &output, metadata.len());
    println!("");

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
        let path: String = json_path.to_string_lossy().into();
        json_data = fs::read_to_string(path)?;
    }

    let mut template_file_path = "".to_string();
    if let Some(template_path) = cli.template.as_deref() {
        template_file_path = template_path.to_string_lossy().into();
    }

    let mut output = "".to_string();
    if let Some(o) = cli.output.as_deref() {
        output = o.to_string_lossy().into();
    }

    let mut template_id_from_opt_remove = "";
    if let Some(t_id) = cli.remove_template.as_deref() {
        template_id_from_opt_remove = t_id;
    }

    let mut template_id_from_opt_download = "";
    if let Some(t_id) = cli.download_template.as_deref() {
        template_id_from_opt_download = t_id;
    }

    let api_token = ApiJsonToken::new(token)?;

    let carbone_sdk = Carbone::new(&config, &api_token)?;

    if  !template_file_path.is_empty() && !json_data.is_empty() && !output.is_empty(){
        generate_template(carbone_sdk, &template_file_path, json_data, output.as_str())?;
    }

    if !template_id_from_opt_remove.is_empty() {
        let template_id = TemplateId::new(template_id_from_opt_remove.to_string())?;
        delete_template(&config, &api_token, template_id)?;
    }

    if !template_id_from_opt_download.is_empty() && !output.is_empty() {
        let template_id = TemplateId::new(template_id_from_opt_download.to_string())?;
        download_template(&config, &api_token, template_id, output.as_str())?;
    }

    if cli.update {
        upload_template(&config, &api_token, &template_file_path)?;
    }

    Ok(())
}