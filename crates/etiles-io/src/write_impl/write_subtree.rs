use crate::Error;
use crate::write_impl::STRING_PADDING_CHARACTER;
use bincode::{Decode, Encode, config};
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use ecoord::octree::VecOctantIndexExt;
use ecoord::octree::{OctantIndex, Octree};
use etiles_core::Vertex;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::io::Write;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct SubtreeBinaryHeader {
    pub magic: [char; 4],
    pub version: u32,
    pub json_byte_length: u64,
    pub binary_byte_length: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub byte_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    pub buffer: u32,
    pub byte_offset: u32,
    pub byte_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<Constant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Constant {
    Unavailable = 0,
    Available = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtree {
    buffers: Vec<Buffer>,
    buffer_views: Vec<BufferView>,
    tile_availability: Availability,
    content_availability: Vec<Availability>,
    child_subtree_availability: Availability,
}

impl Subtree {
    pub fn encode_as_bytes(&self) -> Vec<u8> {
        let mut encoded_data = serde_json::to_vec(&self).unwrap();
        let current_length = encoded_data.len();
        let padding_length = 8 - (current_length % 8);
        // let padding = " ".as_bytes();

        encoded_data.append(&mut vec![STRING_PADDING_CHARACTER; padding_length]);
        encoded_data
    }
}

pub fn write_subtree<W: Write>(
    writer: &mut W,
    base_octant_index: OctantIndex,
    levels_per_subtree: usize,
    content_octree: &Octree<Vertex>,
) -> Result<(), Error> {
    let availability_info =
        get_availability_buffer(base_octant_index, levels_per_subtree, content_octree);

    let subtree_document = Subtree {
        buffers: vec![Buffer {
            byte_length: availability_info.get_combined_buffer().len() as u32,
            ..Default::default()
        }],
        buffer_views: availability_info.get_buffer_views(),
        tile_availability: availability_info.get_tile_availability(),
        content_availability: availability_info.get_content_availability(),
        child_subtree_availability: availability_info.get_child_subtree_availability(),
    };
    let encoded_subtree_json = subtree_document.encode_as_bytes();

    let subtree_binary_header = SubtreeBinaryHeader {
        magic: ['s', 'u', 'b', 't'],
        version: 1,
        json_byte_length: encoded_subtree_json.len() as u64,
        binary_byte_length: availability_info.get_combined_buffer().len() as u64,
    };
    let config = config::standard().with_fixed_int_encoding();
    let encoded_subtree_binary_header: Vec<u8> =
        bincode::encode_to_vec(subtree_binary_header, config).unwrap();

    writer
        .write_all(&encoded_subtree_binary_header)
        .expect("should work");
    writer
        .write_all(&encoded_subtree_json)
        .expect("should work");
    writer
        .write_all(&availability_info.get_combined_buffer())
        .expect("should work");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AvailabilityInfo {
    tile: AvailabilityRecord,
    content: AvailabilityRecord,
    child_subtree: AvailabilityRecord,
}

impl AvailabilityInfo {
    pub fn get_combined_buffer(&self) -> Vec<u8> {
        [
            self.tile.get_padded_buffer(),
            self.content.get_padded_buffer(),
            self.child_subtree.get_padded_buffer(),
        ]
        .concat()
    }

    pub fn get_buffer_views(&self) -> Vec<BufferView> {
        let mut buffer_views = vec![
            BufferView {
                buffer: 0,
                byte_offset: 0,
                byte_length: self.tile.buffer().len() as u32,
                name: None,
            },
            BufferView {
                buffer: 0,
                byte_offset: self.tile.get_padded_buffer().len() as u32,
                byte_length: self.content.buffer().len() as u32,
                name: None,
            },
        ];
        if !self.child_subtree.buffer().is_empty() {
            buffer_views.push(BufferView {
                buffer: 0,
                byte_offset: (self.tile.get_padded_buffer().len()
                    + self.content.get_padded_buffer().len()) as u32,
                byte_length: self.child_subtree.buffer().len() as u32,
                name: None,
            })
        }

        buffer_views
    }

    pub fn get_tile_availability(&self) -> Availability {
        Availability {
            bitstream: Some(0),
            available_count: Some(self.tile.count),
            constant: None,
        }
    }

    pub fn get_content_availability(&self) -> Vec<Availability> {
        vec![Availability {
            bitstream: Some(1),
            available_count: Some(self.content.count),
            constant: None,
        }]
    }

    pub fn get_child_subtree_availability(&self) -> Availability {
        if self.child_subtree.buffer().is_empty() {
            Availability {
                bitstream: None,
                available_count: None,
                constant: Some(Constant::Unavailable),
            }
        } else {
            Availability {
                bitstream: Some(2),
                available_count: Some(self.child_subtree.count),
                constant: None,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AvailabilityRecord {
    bit_buffer: BitVec<u8, Lsb0>,
    count: u32,
}

impl AvailabilityRecord {
    pub fn buffer(&self) -> Vec<u8> {
        self.bit_buffer.clone().into_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get_padded_buffer(&self) -> Vec<u8> {
        if self.buffer().len() % 8 == 0 {
            return self.buffer();
        }

        let padding_buffer = vec![0u8; 8 - (self.buffer().len() % 8)];
        [self.buffer(), padding_buffer].concat()
    }
}

fn get_availability_buffer(
    base_octant_index: OctantIndex,
    levels_per_subtree: usize,
    content_octree: &Octree<Vertex>,
) -> AvailabilityInfo {
    let tile = get_tile_availability_buffer(base_octant_index, levels_per_subtree, content_octree);
    let content =
        get_content_availability_buffer(base_octant_index, levels_per_subtree, content_octree);
    let child_subtree = get_child_subtree_availability_buffer(
        base_octant_index,
        levels_per_subtree,
        content_octree,
    );

    AvailabilityInfo {
        tile,
        content,
        child_subtree,
    }
}

fn get_tile_availability_buffer(
    base_octant_index: OctantIndex,
    levels_per_subtree: usize,
    content_octree: &Octree<Vertex>,
) -> AvailabilityRecord {
    let morton_indices: Vec<(OctantIndex, u64)> = (0..levels_per_subtree)
        .flat_map(|l| {
            base_octant_index
                .get_descendents(l as u32)
                .sort_by_morton_indices()
                .expect("should work")
        })
        .collect();

    let mut available_cell_count: u32 = 0;
    let mut bit_buffer: BitVec<u8, Lsb0> = BitVec::new();
    for current_morton_index in morton_indices {
        let current_availability = content_octree
            .occupancy_graph()
            .is_cell_occupied(current_morton_index.0);
        available_cell_count += current_availability as u32;
        bit_buffer.push(current_availability);
    }

    // https://docs.ogc.org/cs/22-025r4/22-025r4.html#toc142
    debug_assert_eq!(
        bit_buffer.len() as u32,
        (8_u32.pow(levels_per_subtree as u32) - 1) / 7,
        "Wrong tile length"
    );
    AvailabilityRecord {
        bit_buffer,
        count: available_cell_count,
    }
}

fn get_content_availability_buffer(
    base_octant_index: OctantIndex,
    levels_per_subtree: usize,
    content_octree: &Octree<Vertex>,
) -> AvailabilityRecord {
    let morton_indices: Vec<(OctantIndex, u64)> = (0..levels_per_subtree)
        .flat_map(|l| {
            base_octant_index
                .get_descendents(l as u32)
                .sort_by_morton_indices()
                .expect("should work")
        })
        .collect();

    let mut content_availability_cell_count: u32 = 0;
    let mut bit_buffer: BitVec<u8, Lsb0> = BitVec::new();
    for current_morton_index in morton_indices {
        let current_availability = content_octree.contains_content_cells(current_morton_index.0);
        content_availability_cell_count += current_availability as u32;
        bit_buffer.push(current_availability);
    }

    // https://docs.ogc.org/cs/22-025r4/22-025r4.html#toc142
    debug_assert_eq!(
        bit_buffer.len() as u32,
        (8_u32.pow(levels_per_subtree as u32) - 1) / 7,
        "Wrong content length"
    );
    AvailabilityRecord {
        bit_buffer,
        count: content_availability_cell_count,
    }
}

fn get_child_subtree_availability_buffer(
    base_octant_index: OctantIndex,
    levels_per_subtree: usize,
    content_octree: &Octree<Vertex>,
) -> AvailabilityRecord {
    if !content_octree
        .occupancy_graph()
        .is_cell_occupied(base_octant_index)
    {
        panic!("must be occupied in the content cells");
    }
    let morton_indices: Vec<(OctantIndex, u64)> = base_octant_index
        .get_descendents(levels_per_subtree as u32)
        .sort_by_morton_indices()
        .expect("should work");

    let mut available_cell_count: u32 = 0;
    let mut bit_buffer: BitVec<u8, Lsb0> = BitVec::new();
    for current_morton_index in morton_indices {
        let current_availability = content_octree
            .occupancy_graph()
            .is_cell_occupied(current_morton_index.0);
        available_cell_count += current_availability as u32;
        bit_buffer.push(current_availability);
    }

    // https://docs.ogc.org/cs/22-025r4/22-025r4.html#toc142
    debug_assert_eq!(
        bit_buffer.len() as u32,
        8_u32.pow(levels_per_subtree as u32),
        "Wrong tile length"
    );
    AvailabilityRecord {
        bit_buffer,
        count: available_cell_count,
    }
}
