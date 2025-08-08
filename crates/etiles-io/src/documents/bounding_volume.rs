use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoundingVolume {
    Box([f64; 12]),
    Region([f64; 6]),
}
