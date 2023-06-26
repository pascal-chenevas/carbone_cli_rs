use clap::Parser;
use serde_json::json;

use std::env;

use bytes::Bytes;

use std::path::PathBuf;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::template::*;
use carbone_sdk_rs::render::RenderOptions;
use carbone_sdk_rs::carbone::Carbone;
use carbone_sdk_rs::errors::*;
use carbone_sdk_rs::types::*;

mod types;

use types::*;

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

fn generate_template(carbone_sdk: Carbone, template_file_path: &String, json_data: String, output: &str) -> Result<GenerateReportResult, CarboneError> {
    
    let render_options = RenderOptions::new(json_data)?;
    let template_file = TemplateFile::new(template_file_path.to_owned())?;

    let generate_report_result = match carbone_sdk.generate_report_with_file(&template_file, render_options, "") {
        Ok(report_content) => {
            let generate_report_result = match write_file(&report_content, output) {
                Ok(bw) => GenerateReportResult::new(true, output.to_string(), bw, None),
                Err(e) => GenerateReportResult::new(false, output.to_string(), 0, Some(e.to_string())),
            };
            generate_report_result
        },
        Err(e) => GenerateReportResult::new(false, output.to_string(), 0, Some(e.to_string())),
    };

    Ok(generate_report_result)
}

fn upload_template(config: &Config, api_token: &ApiJsonToken, template_file_path: &String) -> UploadResult {
   
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
    upload_result
}

fn download_template(config: &Config, api_token: &ApiJsonToken, template_id: TemplateId, output: &str) -> Result<DownloadResult, CarboneError> {
   
    let template = Template::new(config, api_token);
    let download_result = match template.download(template_id) {
        Ok(content) => {
            let dr = match write_file(&content, output) {
                Ok(bw) => DownloadResult::new(output.to_owned(), true, bw, None),
                Err(e) => DownloadResult::new(output.to_owned(), false, 0, Some(e.to_string())),
            };
            dr
        },
        Err(e) => DownloadResult::new(output.to_owned(), false, 0, Some(e.to_string())),
    };
    
    Ok(download_result)
}

fn delete_template(config: &Config, api_token: &ApiJsonToken, template_id: TemplateId) -> DeleteResult {
   
    let template = Template::new(config, api_token);
    match template.delete(template_id.clone()) {
        Ok(_) => DeleteResult::new(true, Some(template_id.as_str().to_string()), None),
        Err(e) => DeleteResult::new(false, Some(template_id.as_str().to_string()), Some(e.to_string())),
    }
}

fn write_file(content: &Bytes, output: &str) -> Result<u64, CarboneError> {

    let mut file = File::create(output)?;
    file.write_all(content)?;

    let metadata = fs::metadata(output)?;

    Ok(metadata.len())
}

fn get_path_from_option(option: Option<PathBuf>) -> String {

    let mut path = "".to_string();
    if let Some(path_from_option) = option.as_deref() {
        path = path_from_option.to_string_lossy().into();
    }
    path
}

fn get_id_from_option(option: Option<String>) -> String {
    match  option.as_deref() {
        Some(id) => id.to_string(),
        None=> "".to_string(),
    }
}

fn load_config(option: Option<PathBuf>) -> Config {

    let file_path_from_opt_config = get_path_from_option(option);

    match Config::from_file(file_path_from_opt_config.as_str()) {
        Ok(c) => c,
        Err(_) => Default::default(),
    }
}

fn main() -> Result<(), CarboneError> {
    
    let cli = Cli::parse();

    let token =  match env::var("CARBONE_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            println!("\nEnvironment Variable `CARBONE_TOKEN` is not set!\n");
            std::process::exit(1)
        }
    };

    let config = load_config(cli.config);

    let json_path = get_path_from_option(cli.json);
    let mut json_data = "".to_string();
    if !json_path.is_empty() {
        json_data = fs::read_to_string(json_path)?;
    }

    let template_file_path = get_path_from_option(cli.template);

    let output = get_path_from_option(cli.output);

    let template_id_from_opt_remove = get_id_from_option(cli.remove_template);

    let template_id_from_opt_download = get_id_from_option(cli.download_template);

    let api_token = ApiJsonToken::new(token)?;

    let carbone_sdk = Carbone::new(&config, &api_token)?;

    if cli.update {
        let upload_result = upload_template(&config, &api_token, &template_file_path);
        let json = json!(upload_result);
        println!("{:#}", json);
    }

    if  !template_file_path.is_empty() && !json_data.is_empty() && !output.is_empty(){
        let generate_report_result = generate_template(carbone_sdk, &template_file_path, json_data, output.as_str())?;
        let json = json!(generate_report_result);
        println!("{:#}", json);
    }

    if !template_id_from_opt_download.is_empty() && !output.is_empty() {
        let template_id = TemplateId::new(template_id_from_opt_download.to_string())?;
        let download_result = download_template(&config, &api_token, template_id, output.as_str())?;
        let json = json!(download_result);
        println!("{:#}", json);
    }

    if !template_id_from_opt_remove.is_empty() {
        let template_id = TemplateId::new(template_id_from_opt_remove.to_string())?;
        let delete_result = delete_template(&config, &api_token, template_id);
        let json = json!(delete_result);
        println!("{:#}", json);
    }

    Ok(())
}