use std::error::Error;
use std::fs::{create_dir_all,write};
use std::path::{PathBuf};

use cliclack::spinner;

use crate::config::{ProjectConfig, Template};


pub fn generate(config: &ProjectConfig) -> Result<(),Box<dyn Error>> {
    let spinner = spinner();

    spinner.start("Creating your project...");

    match write_project_files(config) {
        Ok(()) => {},
        Err(e) =>{
            spinner.stop("Failed to create the project");
            return Err(e)
        }
    }

    spinner.stop("Your project has been created successfuly!");
    Ok(())
}

fn write_project_files(config: &ProjectConfig) -> Result<(),Box<dyn Error>> {
    let dir = PathBuf::from(&config.project_name);

    let src_path = dir.join("src");
    create_dir_all(&src_path)?;

    let main_rs_content = match config.template {
        Template::BareMetal => include_str!("templates/main.baremetal.rs.tmpl"),
        Template::Embassy => include_str!("templates/main.embassy.rs.tmpl")
    }; 

    write(src_path.join("main.rs"),main_rs_content)?;

    let cargo_toml_content = include_str!("templates/Cargo.toml.tmpl");
    let cargo_toml_content = cargo_toml_content.replace("{{project_name}}", &config.project_name);
    write(dir.join("Cargo.toml"), cargo_toml_content)?;

    Ok(())
}