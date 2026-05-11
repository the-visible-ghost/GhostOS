extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use uefi::cstr16;

use crate::ghost::Ghost;

pub struct Config<'a> {
    pub timeout: i32,
    pub default: u8,
    pub entries: Vec<Entry<'a>>,
}

pub struct Entry<'a> {
    pub title: &'a str,
    pub callback: Option<&'a str>,
    pub submenu: Option<Vec<Entry<'a>>>,
}

pub fn new<'a>() -> Config<'a> {
    return Config {
        timeout: -1,
        default: 0,
        entries: vec![
            Entry {
                title: "Arch Linux",
                callback: Some(""),
                submenu: None,
            },
            Entry {
                title: "Reboot / PowerOff",
                callback: None,
                submenu: Some(vec![
                    Entry {
                        title: "Reboot",
                        callback: Some("reboot"),
                        submenu: None,
                    },
                    Entry {
                        title: "PowerOff",
                        callback: Some("poweroff"),
                        submenu: None,
                    },
                ]),
            },
        ],
    };
}

pub fn load<'a>(ghost: &mut Ghost) -> Option<Config<'a>> {
    match &mut ghost.fs {
        Some(fs) => {
            let res = fs.read_to_string(cstr16!("\\ghost\\boot.ghost"));
            match res.is_err() {
                false => parse(res.unwrap().as_str()),
                true => None,
            }
        }
        None => None,
    }
}

pub fn parse<'a>(data: &str) -> Option<Config<'a>> {
    data;
    None
}
