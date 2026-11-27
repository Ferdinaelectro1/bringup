use std::collections::HashMap;
use std::error::Error;
use std::format;
use std::fs::{create_dir_all,write};
use std::path::{PathBuf};
use std::process::{Command};

use cliclack::{note, spinner};
use serde::{Deserialize};
use minijinja::{context};

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
    let env = build_env()?;
    let dir = PathBuf::from(&config.project_name);

    let src_path = dir.join("src");
    create_dir_all(&src_path)?;

    let new_content = env.get_template("main_rs_embassy")?.render(context! {
        logging => !config.features.is_empty()
    })?;

    let main_rs_content = match config.template {
        Template::BareMetal => include_str!("templates/main.baremetal.rs.tmpl").to_string(),
        Template::Embassy => {
            new_content
        }
    }; 

    write(src_path.join("main.rs"),main_rs_content)?;

    //Generate .gitignore
    write(dir.join(".gitignore"), "/target\n")?;

    //Generate Cargo.toml
    let versions = load_versions()?;
    let cargo_toml_tmpl = env.get_template("cargo_toml_tmpl")?;
    let cargo_toml_new_content = cargo_toml_tmpl.render(context! {
        project_name => &config.project_name,
        chip_feature => &config.target.chip_feature(),
        version => versions.package.version,
        edition => versions.package.edition,
        dependencies => versions.dependencies,
        logging => !config.features.is_empty()
    })?;
    write(dir.join("Cargo.toml"), cargo_toml_new_content)?;

    //Generate config.toml
    let config_toml_new_content = env.get_template("config_toml_tmpl")?.render(context! {
        target_runner => &config.target.runner(),
        rust_target => &config.target.rust_target(),
        logging => !config.features.is_empty()
    })?;
    let dot_cargo_dir = dir.join(".cargo");
    create_dir_all(&dot_cargo_dir)?;
    write(dot_cargo_dir.join("config.toml"),config_toml_new_content)?;


    //Generate build.rs
    let build_rs_new_content = env.get_template("build_rs_tmpl")?.render(context! {
        has_boot2 => config.target == Target::Pico,
        logging   => !config.features.is_empty()
    })?;
    write(dir.join("build.rs"), build_rs_new_content)?;
    
    match config.target {
        Target::Pico => {
            //Generate memory.x
            let memory_x_content = include_str!("templates/pico/memory.x.tmpl");
            write(dir.join("memory.x"), memory_x_content)?;
        },
        Target::Pico2 => {
            //Generate memory.x
            let memory_x_content = include_str!("templates/pico2/memory.x.tmpl");
            write(dir.join("memory.x"), memory_x_content)?;
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

#[derive(Deserialize,Debug)]
pub struct Package {
    pub version: String,
    pub edition: String
}

#[derive(Deserialize,Debug)]
pub struct Versions {
    pub package: Package,
    pub dependencies: HashMap<String,String>
}

pub fn load_versions() -> Result<Versions,Box<dyn Error>>{
    let version_file_content = include_str!("templates/version.toml");
    let v : Versions = toml::from_str(version_file_content)?;
    Ok(v)
}

fn build_env<'a>() -> Result<minijinja::Environment<'a>,Box<dyn Error>> {
    let mut env = minijinja::Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    //Add Cargo.toml template
    let cargo_toml_content = include_str!("templates/Cargo.toml.tmpl");
    env.add_template("cargo_toml_tmpl", cargo_toml_content)?;


    //Add config.toml template
    let config_toml_content = include_str!("templates/config.toml.tmpl");
    env.add_template("config_toml_tmpl", config_toml_content)?;

    //Add build.rs template
    let build_rs_content: &str = include_str!("templates/build.rs.tmpl");
    env.add_template("build_rs_tmpl", build_rs_content)?;

    //Add main.rs embassy template
    let main_rs_embassy_content = include_str!("templates/main.embassy.rs.tmpl");
    env.add_template("main_rs_embassy",main_rs_embassy_content)?;
    
    Ok(env)
}