use super::logger::JormungandrLogger;
use crate::common::configuration::jormungandr_config::JormungandrConfig;
use crate::common::jcli_wrapper;
use crate::common::{process_assert, process_utils};
use std::path::PathBuf;
use std::process::Child;

#[derive(Debug)]
pub struct JormungandrProcess {
    pub child: Child,
    pub logger: JormungandrLogger,
    pub config: JormungandrConfig,
    description: String,
}

impl JormungandrProcess {
    pub fn from_config(child: Child, config: JormungandrConfig) -> Self {
        JormungandrProcess::new(
            child,
            String::from("Jormungandr node"),
            config.log_file_path.clone(),
            config,
        )
    }

    pub fn new(
        child: Child,
        description: String,
        log_file_path: PathBuf,
        config: JormungandrConfig,
    ) -> Self {
        JormungandrProcess {
            child: child,
            description: description,
            logger: JormungandrLogger::new(log_file_path.clone()),
            config: config,
        }
    }

    pub fn assert_no_errors_in_log(&self) {
        let error_lines = self.logger.get_lines_with_error().collect::<Vec<String>>();

        assert_eq!(
            error_lines.len(),
            0,
            "there are some errors in log ({:?}): {:?}",
            self.logger.log_file_path,
            error_lines
        );
    }
}

impl Drop for JormungandrProcess {
    fn drop(&mut self) {
        jcli_wrapper::assert_shutdown_node(&self.config.get_node_address());
        self.logger.print_error_and_invalid_logs();
        self.logger.print_logs_if_contain_error();    }
}
