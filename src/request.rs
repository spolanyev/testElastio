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
        let (Some(mut command), Some(parameter)) = (iterator.next(), iterator.next()) else {
            return Err(());
        };

        let date = {
            let mut result = None;
            if let Some(date) = iterator.next() {
                result = Some(date);
            }
            result
        };

        command = command.to_lowercase();

        Ok(Request {
            command,
            parameter,
            date,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn test_without_date() {
        let arguments: Box<dyn Iterator<Item = String>> = Box::new(
            vec![
                "path_will_be_skipped".to_owned(),
                "configure".to_owned(),
                "Venus".to_owned(),
            ]
            .into_iter(),
        );
        let request = Request::try_from(arguments).unwrap();

        assert_eq!(None, request.get_date());
        assert_eq!("Venus".to_string(), request.get_parameter());
        assert_eq!("configure".to_string(), request.get_command());
    }

    #[test]
    fn test_with_date() {
        let arguments: Box<dyn Iterator<Item = String>> = Box::new(
            vec![
                "path_will_be_skipped".to_owned(),
                "configure".to_owned(),
                "Venus".to_owned(),
                "2022-12-04".to_string(),
            ]
            .into_iter(),
        );
        let request = Request::try_from(arguments).unwrap();

        assert_eq!(Some("2022-12-04".to_string().borrow()), request.get_date());
        assert_eq!("Venus".to_string(), request.get_parameter());
        assert_eq!("configure".to_string(), request.get_command());
    }
}
