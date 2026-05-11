use uefi::{Handle, boot, fs::FileSystem};

pub fn init(image_handle: Handle) -> Option<FileSystem> {
    match boot::get_image_file_system(image_handle) {
        Ok(fs_proto) => Some(FileSystem::new(fs_proto)),
        _ => None,
    }
}
