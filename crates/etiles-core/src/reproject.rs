use crate::error::Error;
use epoint::PointCloud;
use eproj::{Projector, SpatialReferenceIdentifier};
use nalgebra::Point3;
use rayon::prelude::*;

pub fn reproject_point_cloud(
    mut point_cloud: PointCloud,
    from: SpatialReferenceIdentifier,
    to: SpatialReferenceIdentifier,
) -> Result<PointCloud, Error> {
    let all_points = point_cloud.point_data.get_all_points();
    let num_threads = std::cmp::max(rayon::current_num_threads(), 1);
    let chunk_size = all_points.len().div_ceil(num_threads);

    let projected_points: Vec<Point3<f64>> = all_points
        .par_chunks(chunk_size)
        .flat_map(|x| {
            let projector = Projector::new(from, to).unwrap();

            projector.convert_points(x.to_vec()).unwrap()
        })
        .collect();
    point_cloud.update_points(projected_points, None)?;

    Ok(point_cloud)
}
