#![feature(let_chains)]

use std::rc::Rc;

mod models;
mod db;

use db::*;
use crate::io_utils::{get_user_input, wait_for_key_press};
use crate::models::Action;
use crate::navigator::Navigator;

mod ui;
mod io_utils;
mod navigator;

fn main() {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        let page = navigator.get_current_page().unwrap();

        if let Err(error) = page.draw_page() {
            println!("Error rendering page: {}\nPress any key to continue...", error);
            wait_for_key_press();
        };

        let user_input = get_user_input(); // TODO: doesn't work
        let action_result = page.handle_input(user_input.as_str());
        if let Err(error) = &action_result {
            println!("Error action: {}\nPress any key to continue...", error);
            wait_for_key_press();
        };

        let action_opt = action_result.unwrap();

        match action_opt {
            Some(action) => {
                if action == Action::Exit {
                    break;
                } else {
                    println!("action: {:?}", action);

                    if let Err(error) = navigator.handle_action(action) {
                        println!("Error navigation: {}\nPress any key to continue...", error);
                        wait_for_key_press();
                    };
                }
            },
            None => ()
        }
    }
}
