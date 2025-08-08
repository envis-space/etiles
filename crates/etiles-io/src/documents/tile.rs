use crate::documents::bounding_volume::BoundingVolume;
use crate::documents::content::Content;
use crate::documents::implicit_tiling::{ImplicitTiling, SubdivisionScheme, Subtrees};
use etiles_core::BoundingCube;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub geometric_error: f64,
    pub content: Content,
    pub bounding_volume: BoundingVolume,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Tile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<[f64; 16]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refine: Option<Refinement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_tiling: Option<ImplicitTiling>,
}

impl Tile {
    pub fn new(geometric_error: f64, path: &PathBuf, bounding_cube: &BoundingCube) -> Self {
        Tile {
            geometric_error,
            content: Content {
                uri: path.to_str().unwrap().to_string(),
            },
            bounding_volume: BoundingVolume::Box(bounding_cube.bounding_array()),
            children: Vec::new(),
            transform: None,
            refine: Some(Refinement::Add),
            implicit_tiling: None,
        }
    }

    pub fn new_implicit_tile(
        geometric_error: f64,
        _path: &PathBuf,
        bounding_cube: &BoundingCube,
    ) -> Self {
        Self {
            geometric_error,
            content: Content {
                uri: "content/content_{level}__{x}_{y}_{z}.glb".to_string(),
            },
            bounding_volume: BoundingVolume::Box(bounding_cube.bounding_array()),
            children: Vec::new(),
            transform: None,
            refine: Some(Refinement::Add),
            implicit_tiling: Some(ImplicitTiling {
                subdivision_scheme: SubdivisionScheme::Octree,
                subtree_levels: 3,
                available_levels: 6,
                subtrees: Subtrees {
                    uri: "subtrees/{level}.{x}.{y}.{z}.subtree".to_string(),
                },
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Refinement {
    Add,
    Replace,
}
