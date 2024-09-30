use config::ConfigTrait;
use logger::LoggerTrait;

pub trait ViewTrait {
    fn display_greeting(&self, name: &str);
    fn ask_name(&self) -> String;
    fn show_config(&self);
}

pub struct View<L: LoggerTrait, C: ConfigTrait> {
    logger: L,
    config: C,
}

impl<L: LoggerTrait, C: ConfigTrait> View<L, C> {
    pub fn new(logger: L, config: C) -> Self {
        Self { logger, config }
    }
}

impl<L: LoggerTrait, C: ConfigTrait> ViewTrait for View<L, C> {
    fn display_greeting(&self, name: &str) {
        self.logger.log("greeting view generated");
        if self.config.is_rude() {
            println!("Get lost {}", name);
        } else {
            println!("Hello {}", name);
        }
    }

    fn ask_name(&self) -> String {
        self.logger.log("view to ask user generated");
        use std::io;
        println!("What is your name?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        input.trim().to_string()
    }

    fn show_config(&self) {
        if self.config.is_rude() {
            self.logger.log("config: rude");
        } else {
            self.logger.log("config: neutral");
        }
    }
}
