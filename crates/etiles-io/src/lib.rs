mod documents;
mod error;
mod write;
mod write_impl;

#[doc(inline)]
pub use write::EtilesWriter;

#[doc(inline)]
pub use write_impl::write::write_tileset_json;

#[doc(inline)]
pub use write_impl::write::write_subtree_info;

#[doc(inline)]
pub use write_impl::write::derive_content_filename;

#[doc(inline)]
pub use write_impl::content::EncodableContent;

#[doc(inline)]
pub use error::Error;

pub const FILE_EXTENSION_ETILES_UNCOMPRESSED: &str = "tar";
pub const FILE_NAME_TILESET_JSON: &str = "tileset.json";

pub const CONTENT_DIRECTORY_PATH: &str = "content/";
pub const SUBTREES_DIRECTORY_PATH: &str = "subtrees/";
pub const LEVELS_PER_SUBTREE: usize = 3;
