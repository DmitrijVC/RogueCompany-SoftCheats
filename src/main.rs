#![allow(non_snake_case)]

extern crate user32;
extern crate autopilot;
extern crate winconsole;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const WINDOW_WIDTH: u16 = 89;
const WINDOW_HEIGHT: u16 = 14;
const WINDOW_HEIGHT_BUFF: u16 = 200;

mod rpc;
mod menu;
mod print;

use std::{thread, ptr, time, env};
use winapi::um::wincon::GetConsoleWindow;
use enigo::{MouseButton, MouseControllable};
use autopilot::geometry::Point;
use serde_json::{Map, Value};
use console::Term;


fn get_config() -> Result<serde_json::Value, ()> {

    let conf_path = std::path::Path::new("config.json");
    if !conf_path.exists() {
        print::error("config.json not found!".to_string());
        return Err(());
    }

    let content;
    match std::fs::read_to_string(conf_path) {
        Ok(result) => content = result,
        Err(_) => {
            print::error("Can't read config file!".to_string());
            return Err(());
        }
    }

    let config: serde_json::Value;
    return match serde_json::from_str(content.as_str()) {
        Ok(result) => {
            config = result;
            Ok(config)
        },
        Err(_) => {
            print::error("Can't parse json file!".to_string());
            Err(())
        }
    }
}

fn prepare_window() { unsafe {
    let window = GetConsoleWindow();
    winconsole::console::set_title(&*format!("SoftCheats - RogueCompany [{}]", VERSION)).unwrap();
    if window != ptr::null_mut() {
        thread::spawn(|| {
            loop {
                Term::stdout().hide_cursor().unwrap();
                winconsole::console::set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
                winconsole::console::set_buffer_size(WINDOW_WIDTH, WINDOW_HEIGHT_BUFF).unwrap();
                thread::sleep(time::Duration::from_millis(500));
            }
        });
    }
} }


fn match_color(value_from: Vec<u8>, value_to: Vec<u16>, tolerance: u16) -> bool{
    let mut index = 0;
    let mut value16;
    for value in value_from {
        value16 = value as u16;
        if value16 < value_to[index] + tolerance {
            if value16 < value_to[index] - tolerance {
                return false;
            }
        } else {
            return false;
        }

        index += 1;
    }

    true
}

// Kinda fucked up with this one.... See line 163
fn is_triggered(point: Point, match_to: Vec<u16>, tolerance: u16) -> bool {
    let mut color = autopilot::screen::get_color(point).unwrap().0.to_vec();
    color.pop();

    if match_color(color, match_to, tolerance) {
        return true;
    }

    false
}


fn main() {

    ansi_term::enable_ansi_support().unwrap();
    prepare_window();
    rpc::RPC::run();

    let config: Map<String, Value>;
    match get_config() {
        Ok(result) => config = result.as_object().unwrap().clone(),
        Err(()) => {
            loop { }
        }
    }

    let key_click;
    if let Ok(result) = i32::from_str_radix(
        config["trigger_click"].to_string().replace("\"", "").replace("0x", "").as_str(),
        16
        ) {
        key_click = result;
    } else {
        print::error("Error when reading trigger_click!".to_string());
        loop { }
    }

    let key_hold;
    if let Ok(result)  = i32::from_str_radix(
        config["trigger_hold"].to_string().replace("\"", "").replace("0x", "").as_str(),
        16
        ) {
        key_hold = result;
    } else {
        print::error("Error when reading trigger_hold!".to_string());
        loop { }
    }

    menu::print_menu();

    /*
        I should replace 960.0 and 540.0 with
        autopilot::screen::size().height / 2
        autopilot::screen::size().width / 2
        but I've never tested it
    */
    let point = autopilot::geometry::Point::new(960.0, 540.0);
    let match_to = [237u16, 237u16, 237u16].to_vec();
    let tolerance = 10u16;
    let mut eni = enigo::Enigo::new();

    let mut triggered = false;

    loop {

        /*
            Changed -32767 to 0 for quicker response,
            but sometimes it's event "too quick"
        */


        // Disable hold
        if triggered == true {
            if is_triggered(point, match_to.clone(), tolerance) {
                eni.mouse_up(MouseButton::Left);
                triggered = false;
            }
        }

        // Hold
        if unsafe { user32::GetAsyncKeyState(key_hold) } != 0 {
            if !is_triggered(point, match_to.clone(), tolerance) {
                triggered = true;
                eni.mouse_down(MouseButton::Left);
            }
        }

        // Click
        else if unsafe { user32::GetAsyncKeyState(key_click) } != 0 {
            if !is_triggered(point, match_to.clone(), tolerance) {
                eni.mouse_click(MouseButton::Left);
            }
        }
    }

}
