use model::ModelTrait;
use view::ViewTrait;

pub struct Controller<M: ModelTrait, V: ViewTrait> {
    model: M,
    view: V,
}

impl<M: ModelTrait, V: ViewTrait> Controller<M, V> {
    pub fn new(model: M, view: V) -> Self {
        Self { model, view }
    }

    pub fn run(&mut self) {
        self.view.show_config();

        let name = self.view.ask_name();
        self.model.store_name(name);

        let stored_name = self.model.get_name();
        self.view.display_greeting(stored_name);
    }
}
