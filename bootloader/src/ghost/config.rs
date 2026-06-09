use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use uefi::cstr16;
use uefi::fs::FileSystem;

pub struct Config<'config> {
    pub timeout: i32,
    pub default: u8,
    pub entries: Vec<Entry<'config>>,
}

pub struct Entry<'entry> {
    pub title: &'entry str,
    pub callback: Option<&'entry str>,
    pub submenu: Option<Vec<Entry<'entry>>>,
}

fn default<'config>() -> Config<'config> {
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

pub fn load<'config>(fs: &mut FileSystem) -> Config<'config> {
    match parse(fs.read_to_string(cstr16!("\\ghost\\boot.ghost")).ok()) {
        Some(data) => data,
        None => default(),
    }
}

pub fn parse<'config>(_data: Option<String>) -> Option<Config<'config>> {
    None
}
