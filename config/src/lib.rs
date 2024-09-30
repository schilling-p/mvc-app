pub trait ConfigTrait {
    fn is_rude(&self) -> bool;
}

pub struct Config {
    rude: bool,
}

impl Config {
    pub fn new(rude: bool) -> Self {
        Self { rude }
    }
}

impl ConfigTrait for Config {
    fn is_rude(&self) -> bool {
        self.rude
    }
}
