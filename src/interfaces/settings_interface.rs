//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;

pub trait SettingsInterface {
    fn set_provider(&self, provider: &str) -> ExecutionResult;
    fn get_provider(&self) -> Result<String, ExecutionResult>;
}
