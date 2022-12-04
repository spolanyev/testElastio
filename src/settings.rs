//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use directories::ProjectDirs;
use std::fs;

pub struct Settings<'a> {
    pub available_providers: [&'a str; 2],
}

impl<'a> Settings<'a> {
    pub fn set_provider(&self, provider: &str) -> ExecutionResult {
        if !self.available_providers.contains(&provider) {
            return ExecutionResult::WrongConfigureCommandParams;
        }

        let Some(project_dirs) = ProjectDirs::from("com", "spolaniev",  "testElastio")  else {
            return ExecutionResult::CannotDefinePreferenceDir;
        };

        let preferences = project_dirs.config_dir().join("preferences");
        if fs::create_dir_all(project_dirs.config_dir()).is_err()
            || fs::write(preferences, provider).is_err()
        {
            return ExecutionResult::CannotSavePreferences;
        }

        println!("{} set", provider);

        ExecutionResult::Ok
    }

    pub fn get_provider(&self) -> Result<String, ExecutionResult> {
        let Some(project_dirs) = ProjectDirs::from("com", "spolaniev",  "testElastio")  else {
            return Err(ExecutionResult::CannotDefinePreferenceDir);
        };

        let preferences = project_dirs.config_dir().join("preferences");

        Ok(match fs::read_to_string(preferences) {
            Ok(provider) => {
                let mut result = "Gismeteo".to_owned();
                if self.available_providers.contains(&provider.as_str()) {
                    result = provider;
                }
                result
            }
            Err(_) => "Gismeteo".to_owned(),
        })
    }
}
