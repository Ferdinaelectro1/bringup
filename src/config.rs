#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Target {
    Pico,
    Pico2,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ide {
    Vscode,
    RustRover,
    Zed,
    Other,
}

#[derive(Debug)]
pub struct ProjectConfig {
    pub project_name: String,
    pub target: Target,
    pub features: Vec<Feature>,
    pub template: Template,
    pub ide: Ide,
}
