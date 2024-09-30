mod lib;

use config::Config;
use lib::Controller;
use logger::ConsoleLogger;
use model::Model;
use view::View;

fn main() {
    let config = Config::new(true);
    let logger = ConsoleLogger;
    let model = Model::new(logger);
    let view = View::new(ConsoleLogger, config);

    let mut controller = Controller::new(model, view);

    controller.run();
}
