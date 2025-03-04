#![deny(warnings)]
#![deny(clippy::all)]
use sauron::*;
use wasm_bindgen::{
    self,
    prelude::*,
    JsCast,
};

use app::{
    App,
    Msg,
};

#[macro_use]
extern crate log;

mod app;

#[wasm_bindgen]
pub struct Client {
    #[allow(unused)]
    program: Program<App, Msg>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        trace!("Do something with the initial state: {}", initial_state);

        let root_node = document().get_element_by_id("web-app").unwrap();

        let app = App::new(0);
        let program = Program::replace_mount(app, &root_node);
        let program_clone = program.clone();
        let clock: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
            program_clone.dispatch(Msg::Clock);
        }));
        window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                clock.as_ref().unchecked_ref(),
                1000,
            )
            .expect("Unable to start interval");
        clock.forget();
        Client { program }
    }
}
