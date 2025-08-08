use crate::documents::asset::{Asset, Version};
use crate::documents::tile::{Refinement, Tile};
use nalgebra::Isometry3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesetDocument {
    pub asset: Asset,
    pub geometric_error: f64,
    pub root: Tile,
}

impl TilesetDocument {
    pub fn new(
        mut root_tile: Tile,
        transform_isometry: Option<Isometry3<f64>>,
        geometric_error: f64,
    ) -> Self {
        let transform_values =
            transform_isometry.map(|i| <[f64; 16]>::try_from(i.to_matrix().as_slice()).unwrap());

        root_tile.transform = transform_values;
        root_tile.refine = Some(Refinement::Add);

        let asset = Asset {
            version: Version::V1_1,
        };
        Self {
            asset,
            geometric_error,
            root: root_tile,
        }
    }
}
