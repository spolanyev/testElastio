//@author Stanislav Polaniev <spolanyev@gmail.com>

pub trait CommandInterface {
    fn get_command(&self) -> &str;
    fn get_parameter(&self) -> &str;
    fn get_date(&self) -> Option<&String>;
}
