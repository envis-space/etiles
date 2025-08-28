use crate::error::Error;
use crate::reproject::reproject_point_cloud;
use ecoord::HasAabb;
use ecoord::octree::Octree;
use epoint::transform::apply_isometry;
use eproj::{Projector, SpatialReferenceIdentifier};
use nalgebra::{Isometry3, Point3, UnitQuaternion};
use palette::Srgb;
use std::f64;
use std::iter::zip;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Point3<f64>,
    pub color: Srgb<f32>,
}

impl HasAabb for Vertex {
    fn center(&self) -> Point3<f64> {
        self.position
    }

    fn min(&self) -> Point3<f64> {
        self.position
    }

    fn max(&self) -> Point3<f64> {
        self.position
    }
}

pub struct Tileset {
    pub tiled_content: Octree<Vertex>,
    pub root_transform: Isometry3<f64>,
    pub root_geometric_error: f64,
    pub geometric_error: f64,
}

impl Tileset {
    pub fn from_point_cloud(
        point_cloud: epoint::PointCloud,
        source_srs: SpatialReferenceIdentifier,
        maximum_points_per_octant: u64,
        seed_number: Option<u64>,
    ) -> Result<Self, Error> {
        let number_of_points = point_cloud.point_data.height();
        let projector = Projector::new(source_srs, SpatialReferenceIdentifier::Epsg4978)?;
        let isometry = Isometry3::from_parts(
            point_cloud.point_data.get_local_center().into(),
            UnitQuaternion::default(),
        );
        let converted_isometry = projector.convert_isometry(isometry)?;
        //info!("Derived isometry: {:?}", &converted_isometry);

        // info!("Start reprojecting");
        let reprojected_point_cloud = reproject_point_cloud(
            point_cloud,
            source_srs,
            SpatialReferenceIdentifier::Epsg4978,
        )?;

        //info!("Start applying isometry");
        let geodetic_transform_isometry = converted_isometry.inverse();
        let local_point_cloud =
            apply_isometry(&reprojected_point_cloud, geodetic_transform_isometry).unwrap();

        let point_cloud_positions = local_point_cloud.point_data.get_all_points();
        let point_cloud_colors: Vec<Srgb<f32>> =
            match local_point_cloud.point_data.get_all_colors().ok() {
                Some(colors) => colors.into_iter().map(|c| c.into_format()).collect(),
                None => {
                    vec![
                        Srgb::<f32>::new(0.83144885, 0.83144885, 0.83144885);
                        local_point_cloud.point_data.height()
                    ]
                }
            };
        let point_cloud_vertices: Vec<Vertex> = zip(point_cloud_positions, point_cloud_colors)
            .map(|(p, c)| Vertex {
                position: p,
                color: c,
            })
            .collect();

        //info!("Start building octree");
        let point_cloud_octree = Octree::new(
            point_cloud_vertices,
            maximum_points_per_octant as usize,
            seed_number,
        )?;

        let root_geometric_error = point_cloud_octree.bounds().bounding_box().diagonal().norm();
        let geometric_error = {
            let bounding_box_volume = point_cloud_octree.bounds().bounding_box().volume();
            let average_spacing = (bounding_box_volume / number_of_points as f64).cbrt();
            let base_scaling = 7.0;
            average_spacing * 2.0f64.sqrt() * base_scaling
        };
        //info!("geometric_error: {geometric_error}");

        Ok(Self {
            tiled_content: point_cloud_octree,
            root_transform: converted_isometry,
            root_geometric_error,
            geometric_error,
        })
    }
}
