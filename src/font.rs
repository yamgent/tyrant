use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use skrifa::{FontRef, raw::FileRef};
use vello::peniko::{Blob, Font};

pub struct DefaultFonts {
    monospace: Font,
}

impl DefaultFonts {
    pub fn new() -> Self {
        Self {
            monospace: get_default_monospace_font().expect("can load default font"),
        }
    }

    pub fn monospace(&self) -> &Font {
        &self.monospace
    }
}

fn find_font<T: AsRef<str>>(font_file_name: T) -> Option<PathBuf> {
    if cfg!(windows) {
        if let Some(global) = env::var_os("windir").map(|windir| {
            Path::new(&windir)
                .join("Fonts")
                .join(font_file_name.as_ref())
        }) {
            if global.exists() {
                return Some(global);
            }
        }

        if let Some(local) = env::var_os("LOCALAPPDATA").map(|appdatadir| {
            Path::new(&appdatadir)
                .join("Microsoft/Windows/Fonts")
                .join(font_file_name.as_ref())
        }) {
            if local.exists() {
                return Some(local);
            }
        }

        None
    } else {
        // TODO: support more OS
        None
    }
}

fn load_font<T: AsRef<str>>(font_file_name: T) -> Option<Font> {
    let font_path = find_font(font_file_name)?;
    let font_bytes = fs::read(font_path).ok()?;
    Some(Font::new(Blob::new(Arc::new(font_bytes)), 0))
}

fn get_default_monospace_font() -> Option<Font> {
    let font_file_name = if cfg!(windows) {
        "CASCADIACODE.TTF"
    } else {
        panic!("OS not supported")
    };

    load_font(font_file_name)
}

pub fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}
