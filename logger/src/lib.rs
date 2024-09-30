pub trait LoggerTrait {
    fn log(&self, message: &str);
}

pub struct ConsoleLogger;

impl LoggerTrait for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG]: {}", message);
    }
}
