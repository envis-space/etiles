use crate::Error;
use crate::write_impl::content::EncodablePosition;
use gltf::json;
use gltf_json::validation::Checked::Valid;
use gltf_json::validation::USize64;
use nalgebra::{Isometry3, Translation, UnitQuaternion, Vector3};
use std::borrow::Cow;
use std::io::Write;
use std::mem;

fn align_to_multiple_of_four(n: &mut usize) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T: bytemuck::NoUninit>(data: &[T]) -> Vec<u8> {
    let byte_slice: &[u8] = bytemuck::cast_slice(data);
    let mut new_vec: Vec<u8> = byte_slice.to_owned();

    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }

    new_vec
}

/// Point cloud is in Epsg4979
pub fn write_gltf_tile<W: Write>(
    writer: &mut W,
    vertex_list: &Vec<etiles_core::Vertex>,
) -> Result<(), Error> {
    let gltf_axis_adjustment_isometry = Isometry3::from_parts(
        Translation::identity(),
        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -std::f64::consts::FRAC_PI_2),
    );

    // info!("Writing tiangle_vertices");
    let encodable_vertices: Vec<EncodablePosition> = vertex_list
        .iter()
        .map(|v| {
            let transformed_point = gltf_axis_adjustment_isometry * v.position;

            EncodablePosition {
                position: [
                    transformed_point.x as f32,
                    transformed_point.y as f32,
                    transformed_point.z as f32,
                ],
                color: [v.color.red, v.color.green, v.color.blue],
            }
        })
        .collect();

    let min = [
        encodable_vertices
            .iter()
            .map(|i| i.position[0])
            .reduce(f32::min)
            .unwrap(),
        encodable_vertices
            .iter()
            .map(|i| i.position[1])
            .reduce(f32::min)
            .unwrap(),
        encodable_vertices
            .iter()
            .map(|i| i.position[2])
            .reduce(f32::min)
            .unwrap(),
    ];
    let max = [
        encodable_vertices
            .iter()
            .map(|i| i.position[0])
            .reduce(f32::max)
            .unwrap(),
        encodable_vertices
            .iter()
            .map(|i| i.position[1])
            .reduce(f32::max)
            .unwrap(),
        encodable_vertices
            .iter()
            .map(|i| i.position[2])
            .reduce(f32::max)
            .unwrap(),
    ];
    // let (min, max) = bounding_coords(&triangle_vertices);

    //info!("Writing buffer");
    let mut root = gltf_json::Root::default();
    let buffer_length = encodable_vertices.len() * mem::size_of::<EncodablePosition>();
    let buffer = root.push(json::Buffer {
        byte_length: USize64::from(buffer_length),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        uri: None,
    });
    let buffer_view = root.push(json::buffer::View {
        buffer,
        byte_length: USize64::from(buffer_length),
        byte_offset: None,
        byte_stride: Some(json::buffer::Stride(mem::size_of::<EncodablePosition>())),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    });
    let positions = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64(0)),
        count: USize64::from(encodable_vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(Vec::from(min))),
        max: Some(json::Value::from(Vec::from(max))),
        name: None,
        normalized: false,
        sparse: None,
    });
    let colors = root.push(json::Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(USize64::from(3 * mem::size_of::<f32>())),
        count: USize64::from(encodable_vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });

    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), positions);
            map.insert(Valid(json::mesh::Semantic::Colors(0)), colors);
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Valid(json::mesh::Mode::Points),
        targets: None,
    };

    let mesh = root.push(json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
        weights: None,
    });

    let node = root.push(json::Node {
        mesh: Some(mesh),
        ..Default::default()
    });

    root.push(json::Scene {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        nodes: vec![node],
    });

    // info!("Writing padded_byte_vector");
    let padded_byte_vector = to_padded_byte_vector(&encodable_vertices);

    //info!("Writing padded_byte_vector");
    let json_string = json::serialize::to_string(&root).expect("Serialization error");
    let mut json_offset = json_string.len();
    align_to_multiple_of_four(&mut json_offset);
    let glb = gltf::binary::Glb {
        header: gltf::binary::Header {
            magic: *b"glTF",
            version: 2,
            length: (json_offset + buffer_length)
                .try_into()
                .expect("file size exceeds binary glTF limit"),
        },
        bin: Some(Cow::Owned(padded_byte_vector)),
        json: Cow::Owned(json_string.into_bytes()),
    };
    // let writer = std::fs::File::create("triangle.glb").expect("I/O error");
    let buf_writer = std::io::BufWriter::new(writer);
    glb.to_writer(buf_writer).expect("glTF binary output error");

    Ok(())
}
