use crate::Error::{InvalidFileExtension, NoFileExtension};
use crate::error::Error;
use crate::{
    CONTENT_DIRECTORY_PATH, FILE_EXTENSION_ETILES_UNCOMPRESSED, LEVELS_PER_SUBTREE,
    SUBTREES_DIRECTORY_PATH,
};
use etiles_core::Tileset;

use crate::write_impl::write::write;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// `EtilesWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EtilesWriter<W: Write> {
    writer: W,
    content_directory_path: PathBuf,
    subtrees_directory_path: PathBuf,
    levels_per_subtree: usize,
}

impl<W: Write> EtilesWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            content_directory_path: CONTENT_DIRECTORY_PATH.into(),
            subtrees_directory_path: SUBTREES_DIRECTORY_PATH.into(),
            levels_per_subtree: LEVELS_PER_SUBTREE,
        }
    }

    pub fn finish(self, tileset: &Tileset) -> Result<(), Error> {
        write(
            self.writer,
            tileset,
            self.content_directory_path,
            self.subtrees_directory_path,
            self.levels_per_subtree,
        )?;

        Ok(())
    }
}

impl EtilesWriter<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_ETILES_UNCOMPRESSED {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file))
    }
}
