extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use uefi::cstr16;
use uefi::fs::FileSystem;

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

fn default<'a>() -> Config<'a> {
    Config {
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
    }
}

pub fn load<'a>(fs: &mut FileSystem) -> Config<'a> {
    match parse(fs.read_to_string(cstr16!("\\ghost\\boot.ghost")).ok()) {
        Some(data) => data,
        None => default(),
    }
}

pub fn parse<'a>(_data: Option<String>) -> Option<Config<'a>> {
    None
}
