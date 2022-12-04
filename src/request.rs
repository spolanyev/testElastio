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

impl TryFrom<Box<dyn Iterator<Item = String>>> for Request {
    type Error = ();

    fn try_from(mut iterator: Box<dyn Iterator<Item = String>>) -> Result<Self, Self::Error> {
        iterator.next(); //skip file path
        let (Some(command), Some(parameter)) = (iterator.next(), iterator.next()) else {
            return Err(());
        };

        let date = {
            let mut result = None;
            if let Some(date) = iterator.next() {
                result = Some(date);
            }
            result
        };

        Ok(Request {
            command,
            parameter,
            date,
        })
    }
}
