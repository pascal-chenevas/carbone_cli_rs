
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
use bytes::Bytes;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::template::*;
use carbone_sdk_rs::render::RenderOptions;
use carbone_sdk_rs::carbone::{Carbone,Result};
use carbone_sdk_rs::types::*;

use crate::app::cli::Cli;
use crate::app::types::*;

pub struct App<'a> {
    cli: &'a Cli,
    carbone: Carbone<'a>,
}

impl <'a>App<'a> {

    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken, cli: &'a Cli) -> Result<Self> {

        let carbone = Carbone::new(config, api_token)?;

        let app = Self {cli, carbone};

        Ok(app)
    }

    pub fn run(&self) -> Result<()> {

        let json_path = self.cli.get_path_from_option(&self.cli.json);

        let mut json_data = "".to_string();
        if !json_path.is_empty() {
            json_data = fs::read_to_string(json_path)?;
        }

        let template_file_path = self.cli.get_path_from_option(&self.cli.template);

        let output = self.cli.get_path_from_option(&self.cli.output);

        let template_id_from_opt_remove = self.cli.get_id_from_option(&self.cli.remove_template);

        let template_id_from_opt_download = self.cli.get_id_from_option(&self.cli.download_template);

        if self.cli.update {
            let upload_result = self.upload_template(&template_file_path);
            let json = json!(upload_result);
            println!("{:#}", json);
        }
    
        if  !template_file_path.is_empty() && !json_data.is_empty() && !output.is_empty(){
            let generate_report_result = self.generate_template(&template_file_path, json_data, output.as_str())?;
            let json = json!(generate_report_result);
            println!("{:#}", json);
        }
    
        if !template_id_from_opt_download.is_empty() && !output.is_empty() {
            let template_id = TemplateId::new(template_id_from_opt_download)?;
            let download_result = self.download_template(template_id, output.as_str())?;
            let json = json!(download_result);
            println!("{:#}", json);
        }
    
        if !template_id_from_opt_remove.is_empty() {
            let template_id = TemplateId::new(template_id_from_opt_remove)?;
            let delete_result = self.delete_template(template_id);
            let json = json!(delete_result);
            println!("{:#}", json);
        }

        Ok(())
    }

    fn generate_template(&self, template_file_path: &String, json_data: String, output: &str) -> Result<GenerateReportResult> {
    
        let render_options = RenderOptions::new(json_data)?;
        let template_file = TemplateFile::new(template_file_path.to_owned())?;
    
        let generate_report_result = match self.carbone.generate_report_with_file(&template_file, render_options, "") {
            Ok(report_content) => {
                match Self::write_file(&report_content, output) {
                    Ok(bw) => GenerateReportResult::new(true, output.to_string(), bw, None),
                    Err(e) => GenerateReportResult::new(false, output.to_string(), 0, Some(e.to_string())),
                }
            },
            Err(e) => GenerateReportResult::new(false, output.to_string(), 0, Some(e.to_string())),
        };
    
        Ok(generate_report_result)
    }
    
    fn upload_template(&self, template_file_path: &String) -> UploadResult {
       
        let upload_result = match TemplateFile::new(template_file_path.to_owned()) {
            Ok(tf) => {
                let result = self.carbone.template().upload(&tf, "".to_string());
                match result {
                    Ok(id) => UploadResult::new(template_file_path.to_owned(), true, Some(id.as_str().to_string()), None),
                    Err(e) => UploadResult::new(template_file_path.to_owned(), false, None, Some(e.to_string()))
                }
            },
            Err(e) =>  UploadResult::new(template_file_path.to_owned(), false, None, Some(e.to_string()))
        };
        upload_result
    }
    
    fn download_template(&self, template_id: TemplateId, output: &str) -> Result<DownloadResult> {
       
        let download_result = match self.carbone.template().download(template_id) {
            Ok(content) => {
                match Self::write_file(&content, output) {
                    Ok(bw) => DownloadResult::new(output.to_owned(), true, bw, None),
                    Err(e) => DownloadResult::new(output.to_owned(), false, 0, Some(e.to_string())),
                }
            },
            Err(e) => DownloadResult::new(output.to_owned(), false, 0, Some(e.to_string())),
        };
        
        Ok(download_result)
    }
    
    fn delete_template(&self, template_id: TemplateId) -> DeleteResult {
       
        match self.carbone.template().delete(template_id.clone()) {
            Ok(_) => DeleteResult::new(true, Some(template_id.as_str().to_string()), None),
            Err(e) => DeleteResult::new(false, Some(template_id.as_str().to_string()), Some(e.to_string())),
        }
    }
    
    fn write_file(content: &Bytes, output: &str) -> Result<u64> {
    
        let mut file = File::create(output)?;
        file.write_all(content)?;
    
        let metadata = fs::metadata(output)?;
    
        Ok(metadata.len())
    }
       
}