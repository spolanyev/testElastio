//@author Stanislav Polaniev <spolanyev@gmail.com>

use crate::execution_result::ExecutionResult;
use crate::interfaces::command_interface::CommandInterface;
use crate::interfaces::settings_interface::SettingsInterface;

pub trait ExecutorChainInterface {
    fn execute(
        &self,
        request: &dyn CommandInterface,
        settings: &dyn SettingsInterface,
    ) -> ExecutionResult;
    fn next(&self) -> Option<&dyn ExecutorChainInterface>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Settings;
    use logtest::Logger;

    #[test]
    fn test_execute() {
        //request
        struct Request {
            command: String,
        }

        impl CommandInterface for Request {
            fn get_command(&self) -> &str {
                self.command.as_ref()
            }

            fn get_parameter(&self) -> &str {
                "" //we don't need it for this test only
            }

            fn get_date(&self) -> Option<&String> {
                None
            }
        }

        //first executor. Has next
        struct FirstExecutor<'a> {
            next: Option<&'a dyn ExecutorChainInterface>,
        }

        impl<'a> ExecutorChainInterface for FirstExecutor<'a> {
            fn execute(
                &self,
                request: &dyn CommandInterface,
                settings: &dyn SettingsInterface,
            ) -> ExecutionResult {
                if "I need Executor I" == request.get_command() {
                    log::info!("Executor I proceeded");
                    return ExecutionResult::Ok;
                }

                if let Some(next_executor) = self.next {
                    return next_executor.execute(request, settings);
                }
                ExecutionResult::Err
            }

            fn next(&self) -> Option<&'a dyn ExecutorChainInterface> {
                self.next
            }
        }

        //second executor. Without next
        struct SecondExecutor {}

        impl ExecutorChainInterface for SecondExecutor {
            fn execute(
                &self,
                request: &dyn CommandInterface,
                _settings: &dyn SettingsInterface,
            ) -> ExecutionResult {
                if "I need Executor II" == request.get_command() {
                    log::info!("Executor II proceeded");
                    return ExecutionResult::Ok;
                }
                log::info!("I do not know who can help!");
                ExecutionResult::Ok
            }

            fn next(&self) -> Option<&dyn ExecutorChainInterface> {
                None
            }
        }

        //start the logger
        let mut logger = Logger::start();

        let settings = Settings {
            available_providers: ["Gismeteo", "Alvares"],
        };

        //target is the second executor
        let request: Box<dyn CommandInterface> = Box::new(Request {
            command: "I need Executor II".to_owned(),
        });

        let second_executor = SecondExecutor {};
        let first_executor = FirstExecutor {
            next: Some(&second_executor),
        };

        //will not find a handler in the first executor but will find in the second with `next` method
        first_executor.execute(request.as_ref(), &settings);
        assert_eq!("Executor II proceeded", logger.pop().unwrap().args());

        //target is the first executor
        let request: Box<dyn CommandInterface> = Box::new(Request {
            command: "I need Executor I".to_owned(),
        });

        //will not find a handler in the second executor and `next` method is empty
        second_executor.execute(request.as_ref(), &settings);
        assert_eq!("I do not know who can help!", logger.pop().unwrap().args());
    }
}
