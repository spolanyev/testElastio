//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::interfaces::command_interface::CommandInterface;

pub struct Request {
    pub command: String,
    pub parameter: String,
    pub date: Option<String>,
}

impl CommandInterface for Request {
    fn get_command(&self) -> &str {
        self.command.as_ref()
    }

    fn get_parameter(&self) -> &str {
        self.parameter.as_ref()
    }

    fn get_date(&self) -> Option<&String> {
        self.date.as_ref()
    }
}
