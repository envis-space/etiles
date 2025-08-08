use ecoord::AxisAlignedBoundingCube;
use eproj::Coordinate3;
use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct BoundingCube {
    center: Point3<f64>,
    width: f64,
}

impl BoundingCube {
    pub fn new(center: Point3<f64>, width: f64) -> Self {
        Self { center, width }
    }

    pub fn from_axis_aligned_bounding_cube(bounding_cube: &AxisAlignedBoundingCube) -> Self {
        Self {
            center: bounding_cube.center(),
            width: bounding_cube.edge_length(),
        }
    }

    pub fn get_lower_bound(&self) -> Point3<f64> {
        self.center - Vector3::new(self.width, self.width, self.width)
    }

    pub fn get_upper_bound(&self) -> Point3<f64> {
        self.center + Vector3::new(self.width, self.width, self.width)
    }

    pub fn center_vector(&self) -> Vector3<f64> {
        self.center.coords
    }

    pub fn get_octant(&self, x_half: bool, y_half: bool, z_half: bool) -> BoundingCube {
        let octant_width = self.width / 2.0;
        let x_sign = if x_half { 1.0 } else { -1.0 };
        let y_sign = if y_half { 1.0 } else { -1.0 };
        let z_sign = if z_half { 1.0 } else { -1.0 };

        let octant_center = self.center
            + Vector3::new(
                octant_width * x_sign,
                octant_width * y_sign,
                octant_width * z_sign,
            );

        Self::new(octant_center, octant_width)
    }

    pub fn x_axis(&self) -> Vector3<f64> {
        Vector3::new(self.width / 2.0, 0.0, 0.0)
    }

    pub fn y_axis(&self) -> Vector3<f64> {
        Vector3::new(0.0, self.width / 2.0, 0.0)
    }

    pub fn z_axis(&self) -> Vector3<f64> {
        Vector3::new(0.0, 0.0, self.width / 2.0)
    }

    pub fn bounding_array(&self) -> [f64; 12] {
        let center_vec = self.center_vector();
        let x_axis_vec = self.x_axis();
        let y_axis_vec = self.y_axis();
        let z_axis_vec = self.z_axis();

        [
            center_vec.x,
            center_vec.y,
            center_vec.z,
            x_axis_vec.x,
            x_axis_vec.y,
            x_axis_vec.z,
            y_axis_vec.x,
            y_axis_vec.y,
            y_axis_vec.z,
            z_axis_vec.x,
            z_axis_vec.y,
            z_axis_vec.z,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingRegion {
    south_west_min_height: Coordinate3,
    north_east_max_height: Coordinate3,
}

impl BoundingRegion {
    pub fn new(south_west_min_height: Coordinate3, north_east_max_height: Coordinate3) -> Self {
        Self {
            south_west_min_height,
            north_east_max_height,
        }
    }

    pub fn as_array(&self) -> [f64; 6] {
        let south_west_min_height_radian = self.south_west_min_height.to_radians();
        let north_east_max_height_radian = self.north_east_max_height.to_radians();

        [
            south_west_min_height_radian.x(),
            south_west_min_height_radian.y(),
            north_east_max_height_radian.x(),
            north_east_max_height_radian.y(),
            south_west_min_height_radian.z(),
            north_east_max_height_radian.z(),
        ]
    }
}
