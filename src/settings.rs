//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::settings_interface::SettingsInterface;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct Settings<'a> {
    pub available_providers: [&'a str; 2],
}

impl<'a> Settings<'a> {
    fn get_preferences_path() -> Result<PathBuf, ()> {
        let Some(project_dirs) = ProjectDirs::from("com", "spolaniev", "testElastio")  else {
            return Err(());
        };

        let preferences = project_dirs.config_dir().join("preferences");
        Ok(preferences)
    }
}

impl<'a> SettingsInterface for Settings<'a> {
    fn set_provider(&self, provider: &str) -> ExecutionResult {
        if !self.available_providers.contains(&provider) {
            return ExecutionResult::WrongConfigureCommandParams;
        }

        let Ok(preferences) = Self::get_preferences_path() else {
            return ExecutionResult::CannotDefinePreferenceDir;
        };

        if fs::create_dir_all(preferences.parent().expect("We have at least one level")).is_err()
            || fs::write(preferences, provider).is_err()
        {
            return ExecutionResult::CannotSavePreferences;
        }

        println!("{} set", provider);

        ExecutionResult::Ok
    }

    fn get_provider(&self) -> Result<String, ExecutionResult> {
        let Ok(preferences) = Self::get_preferences_path() else {
            return Err(ExecutionResult::CannotDefinePreferenceDir);
        };

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

#[cfg(test)]

mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn set_and_get() {
        let settings = Settings {
            available_providers: ["Mercury", "Venus"],
        };

        settings.set_provider("Mercury");
        assert_eq!(Ok("Mercury".to_string()), settings.get_provider());

        settings.set_provider("Venus");
        assert_eq!(Ok("Venus".to_string()), settings.get_provider());
    }
}
