use crate::modulos::init_app::init;
use crate::modulos::menu::start_menu;

extern crate dirs;

pub fn run_app() {

    init();
    start_menu();

}