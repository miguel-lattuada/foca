use crate::core::executor::Executor;

use super::{cli::File, executor_trait::InterfaceExecutor};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    pub url: String,
    pub duration: u8,
    pub rate: u8,
    pub workers: u8,
}

pub struct FileExecutor<'a> {
    file: &'a File,
}

impl<'a> FileExecutor<'a> {
    pub fn new(file: &'a File) -> Self {
        Self { file: file }
    }

    fn read_file(&self, file: &String) -> String {
        fs::read_to_string(file).unwrap()
    }

    fn parse_yaml(&self, yaml_file: &String) -> Option<Config> {
        let file_contents = self.read_file(yaml_file);
        Some(serde_yaml::from_str::<Config>(&file_contents).unwrap())
    }

    fn parse_json(&self, json_file: &String) -> Option<Config> {
        let file_contents = self.read_file(json_file);
        Some(serde_json::from_str::<Config>(&file_contents).unwrap())
    }
}

impl<'a> InterfaceExecutor for FileExecutor<'a> {
    fn execute(&self) {
        let mut config = None;

        if let Some(yaml) = &self.file.yaml {
            config = self.parse_yaml(yaml)
        }

        if let Some(json) = &self.file.json {
            config = self.parse_json(json);
        }

        if let Some(config) = config {
            Executor {
                duration: config.duration,
                rate: config.rate,
                url: config.url,
                workers: config.workers,
            }
            .execute();
        }
    }
}
