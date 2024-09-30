use logger::LoggerTrait;

pub trait ModelTrait {
    fn store_name(&mut self, name: String);
    fn get_name(&self) -> &str;
}

pub struct Model<L: LoggerTrait> {
    name: String,
    logger: L,
}

impl<L: LoggerTrait> Model<L> {
    pub fn new(logger: L) -> Self {
        Self {
            name: String::new(),
            logger,
        }
    }
}

impl<L: LoggerTrait> ModelTrait for Model<L> {
    fn store_name(&mut self, name: String) {
        self.logger.log("user name stored");
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.logger.log("user name retreived");
        &self.name
    }
}
