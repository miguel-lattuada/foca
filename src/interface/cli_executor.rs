use crate::core::builder::LoadTestBuilder;

use super::{cli::Command, executor_trait::InterfaceExecutor};

pub struct CliExecutor<'a> {
    command: &'a Command,
}

impl<'a> CliExecutor<'a> {
    pub fn new(command: &'a Command) -> Self {
        Self { command: command }
    }
}

impl<'a> InterfaceExecutor for CliExecutor<'a> {
    fn execute(&self) {
        LoadTestBuilder::new()
            .duration(self.command.duration)
            .rate(self.command.rate)
            .url(self.command.url.to_owned())
            .workers(self.command.workers)
            .execute()
    }
}
