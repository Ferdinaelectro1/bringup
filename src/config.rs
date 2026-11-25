use crate::config::Target::{Pico, Pico2};

#[derive(Clone, PartialEq, Eq, Debug,Copy)]
pub enum Target {
    Pico,
    Pico2,
}

impl Target {
    pub fn chip_feature(self) -> String {
        match self {
            Pico => {
                "rp2040".to_string()
            },
            Pico2 => {
                "rp235xa".to_string()
            }
        }
    }

    pub fn runner(self) -> String {
        match self {
            Pico => {
                "elf2uf2-rs -d".to_string()
            },
            Pico2 => {
                "picotool load -u -v -x -t elf".to_string()
            },
        }
    }

    pub fn rust_target(self) -> String {
        match self {
            Pico => {
                "thumbv6m-none-eabi".to_string()
            }
            Pico2 => {
                "thumbv8m.main-none-eabihf".to_string()
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Feature {
    Logging,
    UsbNative,
    Probe,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Template {
    Embassy,
    BareMetal,
}

#[derive(Debug)]
pub struct ProjectConfig {
    pub project_name: String,
    pub target: Target,
    pub features: Vec<Feature>,
    pub template: Template,
}
