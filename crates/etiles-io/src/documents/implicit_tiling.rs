use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitTiling {
    pub subdivision_scheme: SubdivisionScheme,
    pub subtree_levels: u16,
    pub available_levels: u16,
    pub subtrees: Subtrees,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubdivisionScheme {
    Quadtree,
    Octree,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtrees {
    pub uri: String,
}
