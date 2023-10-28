use bytes::Bytes;
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use carbone_sdk_rust::blocking::Carbone;
use carbone_sdk_rust::config::Config;
use carbone_sdk_rust::template::*;
use carbone_sdk_rust::types::Result;
use carbone_sdk_rust::types::*;

use crate::app::cli::Cli;
use crate::app::types::*;

pub struct App<'a> {
    cli: &'a Cli,
    carbone: Carbone<'a>,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config, api_token: &'a ApiJsonToken, cli: &'a Cli) -> Result<Self> {
        let carbone = Carbone::new(config, api_token)?;

        let app = Self { cli, carbone };

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

        let value_from_opt_remove = self.cli.get_value_from_option(&self.cli.remove_template);

        let template_id_from_opt_download =
            self.cli.get_value_from_option(&self.cli.download_template);

        if self.cli.generate_template_id && !template_file_path.is_empty() {
            let template_file = TemplateFile::new(template_file_path.to_owned(), None)?;
            let template_id = template_file.generate_id(None)?;

            let json = json!({
                    "file": template_file_path,
                    "templateId": template_id });

            println!("{:#}", json);
        }

        if self.cli.update {
            let upload_result = self.upload_template(&template_file_path);
            let json = json!(upload_result);
            println!("{:#}", json);
        }

        if !template_file_path.is_empty() && !json_data.is_empty() && !output.is_empty() {
            let generate_report_result =
                self.generate_report(&template_file_path, json_data, output.as_str())?;
            let json = json!(generate_report_result);
            println!("{:#}", json);
        }

        if !template_id_from_opt_download.is_empty() && !output.is_empty() {
            let template_id = TemplateId::new(template_id_from_opt_download)?;
            let download_result = self.download_template(template_id, output.as_str());
            let json = json!(download_result);
            println!("{:#}", json);
        }

        if !value_from_opt_remove.is_empty() {
            let template_id: TemplateId;
            if Path::new(value_from_opt_remove.as_str()).is_file() {
                let template_file = TemplateFile::new(value_from_opt_remove, None)?;
                template_id = template_file.generate_id(None)?;
            } else {
                template_id = TemplateId::new(value_from_opt_remove)?;
            }

            let delete_result = self.delete_template(template_id);
            let json = json!(delete_result);
            println!("{:#}", json);
        }

        Ok(())
    }

    fn generate_report(
        &self,
        template_file_path: &String,
        json_data: String,
        output: &str,
    ) -> Result<JsonResult> {
        let json_data = JsonData::new(json_data)?;
        let template_file = TemplateFile::new(template_file_path.to_owned(), None)?;

        let json_result =
            match self
                .carbone
                .generate_report_with_file(&template_file, json_data, None)
            {
                Ok(report_content) => match Self::write_file(&report_content, output) {
                    Ok(bw) => JsonResult {
                        state: State::Created(true),
                        file: Some(output.to_string()),
                        bytes: Some(bw),
                        template_id: None,
                        error: None,
                    },
                    Err(e) => JsonResult {
                        state: State::Created(false),
                        file: Some(output.to_string()),
                        bytes: Some(0),
                        template_id: None,
                        error: Some(e.to_string()),
                    },
                },
                Err(e) => JsonResult {
                    state: State::Created(false),
                    file: Some(output.to_string()),
                    bytes: Some(0),
                    template_id: None,
                    error: Some(e.to_string()),
                },
            };

        Ok(json_result)
    }

    fn upload_template(&self, template_file_path: &String) -> JsonResult {
        match TemplateFile::new(template_file_path.to_owned(), None) {
            Ok(tf) => {
                let result = self.carbone.upload_template(&tf, None);
                match result {
                    Ok(id) => JsonResult {
                        state: State::Uploaded(true),
                        file: Some(template_file_path.to_owned()),
                        bytes: None,
                        template_id: Some(id.as_str().to_string()),
                        error: None,
                    },
                    Err(e) => JsonResult {
                        state: State::Uploaded(false),
                        file: Some(template_file_path.to_owned()),
                        bytes: None,
                        template_id: None,
                        error: Some(e.to_string()),
                    },
                }
            }
            Err(e) => JsonResult {
                state: State::Uploaded(false),
                file: Some(template_file_path.to_owned()),
                bytes: None,
                template_id: None,
                error: Some(e.to_string()),
            },
        }
    }

    fn download_template(&self, template_id: TemplateId, output: &str) -> JsonResult {
        match self.carbone.download_template(&template_id) {
            Ok(content) => match Self::write_file(&content, output) {
                Ok(bw) => JsonResult {
                    state: State::Downloaded(true),
                    file: Some(output.to_owned()),
                    bytes: Some(bw),
                    template_id: Some(template_id.as_str().to_string()),
                    error: None,
                },
                Err(e) => JsonResult {
                    state: State::Downloaded(false),
                    file: Some(output.to_owned()),
                    bytes: Some(0),
                    template_id: Some(template_id.as_str().to_string()),
                    error: Some(e.to_string()),
                },
            },
            Err(e) => JsonResult {
                state: State::Downloaded(false),
                file: Some(output.to_owned()),
                bytes: Some(0),
                template_id: Some(template_id.as_str().to_string()),
                error: Some(e.to_string()),
            },
        }
    }

    fn delete_template(&self, template_id: TemplateId) -> JsonResult {
        match self.carbone.delete_template(template_id.clone()) {
            Ok(_) => JsonResult {
                state: State::Deleted(true),
                file: None,
                bytes: None,
                template_id: Some(template_id.as_str().to_string()),
                error: None,
            },

            Err(e) => JsonResult {
                state: State::Deleted(false),
                file: None,
                bytes: None,
                template_id: Some(template_id.as_str().to_string()),
                error: Some(e.to_string()),
            },
        }
    }

    fn write_file(content: &Bytes, output: &str) -> Result<u64> {
        let mut file = File::create(output)?;
        file.write_all(content)?;

        let metadata = fs::metadata(output)?;

        Ok(metadata.len())
    }
}
