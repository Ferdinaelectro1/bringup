use crate::config::{Feature, Ide, ProjectConfig, Target, Template};
use cliclack::{input, multiselect, select};
use std::{error::Error, format, path::{Path, PathBuf}};

pub fn ask_config(project_name: Option<String>) -> Result<ProjectConfig, Box<dyn Error>> {
    let project_name = match project_name {
        Some(name) => name,
        None => get_interactive_project_name()?,
    };

    if Path::exists(&PathBuf::from(&project_name)) {
        return Err(format!("Directory '{}' already exists",&project_name).into());
    }

    let selected_target = select("Choose your target platform!")
        .item(Target::Pico, "pico", "")
        .item(Target::Pico2, "pico2", "")
        .interact()?;

    let selected_features = multiselect("Select the features you need")
        .item(Feature::Logging, "Logging (defmt over RTT)", "")
        .item(Feature::Probe, "Debug probe (probe-rs)", "")
        .item(Feature::UsbNative, "Native USB", "")
        .required(false)
        .interact()?;

    let selected_template = select("Select your project template")
        .item(Template::Embassy, "Embassy Project", "")
        .item(Template::BareMetal, "Bare Metal Project", "")
        .interact()?;

    let selected_ide = select("Select your IDE")
        .item(Ide::Vscode, "VSCode", "")
        .item(Ide::RustRover, "RustRover", "")
        .item(Ide::Zed, "Zed", "")
        .item(Ide::Other, "Other", "")
        .interact()?;

    Ok(ProjectConfig {
        project_name,
        target: selected_target,
        features: selected_features,
        template: selected_template,
        ide: selected_ide,
    })
}

fn get_interactive_project_name() -> Result<String, Box<dyn Error>> {
    let name: String = input("What is the name of your project?")
        .placeholder("name")
        .validate(|user_input: &String| {
            if user_input.is_empty() {
                Err("Please enter a project name")
            } else {
                Ok(())
            }
        })
        .interact()?;
    Ok(name)
}
