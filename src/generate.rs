use std::error::Error;
use std::format;
use std::fs::{create_dir_all,write};
use std::path::{PathBuf};
use std::process::{Command};

use cliclack::{note, spinner};

use crate::config::{ProjectConfig, Target, Template};


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

    spinner.stop("Your project has been created successfully!");
    note("", format!("Done! Next steps:\n cd {}\n cargo run --release",config.project_name))?;
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

    //Generate .gitignore
    write(dir.join(".gitignore"), "/target")?;

    //Generate Cargo.toml
    let cargo_toml_content = include_str!("templates/Cargo.toml.tmpl");
    let cargo_toml_content = cargo_toml_content.replace("{{project_name}}", &config.project_name);
    let cargo_toml_content = cargo_toml_content.replace("{{chip_feature}}", &config.target.chip_feature());
    write(dir.join("Cargo.toml"), cargo_toml_content)?;

    //Generate config.toml
    let config_toml_content = include_str!("templates/config.toml.tmpl");
    let config_toml_content = config_toml_content.replace("{{target_runner}}", &config.target.runner());
    let config_toml_content = config_toml_content.replace("{{rust_target}}", &config.target.rust_target());  
    let dot_cargo_dir = dir.join(".cargo");
    create_dir_all(&dot_cargo_dir)?;
    write(dot_cargo_dir.join("config.toml"),config_toml_content)?;

    match config.target {
        Target::Pico => {
            //Generate memory.x
            let memory_x_content = include_str!("templates/pico/memory.x.tmpl");
            write(dir.join("memory.x"), memory_x_content)?;
            //Generate build.rs
            let build_rs_content: &str = include_str!("templates/pico/build.rs.tmpl");
            write(dir.join("build.rs"), build_rs_content)?;
        },
        Target::Pico2 => {
            //Generate memory.x
            let memory_x_content = include_str!("templates/pico2/memory.x.tmpl");
            write(dir.join("memory.x"), memory_x_content)?;
            //Generate build.rs
            let build_rs_content = include_str!("templates/pico2/build.rs.tmpl");
            write(dir.join("build.rs"), build_rs_content)?;
        }
    }

    // Init git
    let output = Command::new("git")
    .arg("init")
    .current_dir(&dir)
    .output()?;

    if !output.status.success() {
        return Err(format!("Failed to initialize git. {}",String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}