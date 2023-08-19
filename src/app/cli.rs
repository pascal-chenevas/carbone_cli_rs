
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli { 6 implementations
    /// a configuration file which contains the api url, timeout and api version
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// json data to be rendered
    #[arg(short, long, value_name = "FILE")]
    pub json: Option<PathBuf>,

    /// template file
    #[arg(short, long, value_name = "FILE")]
    pub template: Option<PathBuf>,

    /// template file
    #[arg(short, long)]
    pub generate_template_id: bool,

    /// output file for the generated report
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// remove a template with the given template_id
    #[arg(short, long, required = false, value_name = "TEMPLATE_ID")]
    pub remove_template: Option<String>,

    /// update a template
    #[arg(short, long, required = false)]
    pub update: bool,

    /// download a template
    #[arg(short, long, required = false, value_name = "TEMPLATE_ID")]
    pub download_template: Option<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn get_path_from_option(&self, option: &Option<PathBuf>) -> String {

        let mut path = "".to_string();
        if let Some(path_from_option) = option.as_deref() {
            path = path_from_option.to_string_lossy().into();
        }
        path
    }

    pub fn get_id_from_option(&self, option: &Option<String>) -> String {
        match  option.as_deref() {
            Some(id) => id.to_string(),
            None=> "".to_string(),
        }
    }

}
