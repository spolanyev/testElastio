//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::app_exit_code::AppExitCode;

pub trait ProviderSettingsInterface {
    fn set_provider(&self, provider: &str) -> AppExitCode;
    fn get_provider(&self) -> Result<String, AppExitCode>;
}
