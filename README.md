# etiles

A Rust library and CLI tool for processing [3D Tiles](https://www.ogc.org/standards/3dtiles/) data.

> Early stage of development. Developed at the [TUM Chair of Geoinformatics](https://github.com/tum-gis). Contributions welcome.

Features:

- Converts point clouds to 3D Tiles 1.1 using octree spatial indexing
- Supports LAS, LAZ, E57, XYZ, and XYZ+Zstandard input formats
- Reprojects from any EPSG coordinate system to ECEF (EPSG:4978) using PROJ
- Encodes RGB colors from input point clouds into GLB tiles
- Outputs implicit tiling subtrees for efficient streaming

---

## etiles-cli

A command-line tool for converting point clouds to 3D Tiles.

### Installation

Download pre-built executables for your platform from the [release page](https://github.com/tum-gis/etiles/releases).

On macOS, you need to remove the quarantine attribute after downloading:

```sh
xattr -d com.apple.quarantine ./etiles-cli
```

Docker:

```sh
docker pull ghcr.io/tum-gis/etiles-cli
```

From source:

```sh
cargo install etiles-cli@0.0.2-alpha.1 # replace with the latest version
```

### Usage

```sh
etiles-cli convert-point-cloud \
  --input-path /path/to/pointcloud.las \
  --output-path /path/to/output.tar \
  --source-crs 25832
```

For a full list of options:

```sh
etiles-cli convert-point-cloud --help
```

Convert an entire directory of point cloud files:

```sh
etiles-cli convert-point-cloud \
  --input-path /path/to/point_clouds/ \
  --output-path /path/to/output.tar \
  --source-crs 25832
```

Docker:

```sh
docker run --rm \
  -v /path/to/data:/data \
  ghcr.io/tum-gis/etiles-cli \
  convert-point-cloud \
  --input-path /data/pointcloud.las \
  --output-path /data/output.tar \
  --source-crs 25832
```

### Options

| Option                        | Default  | Description                                          |
|-------------------------------|----------|------------------------------------------------------|
| `--input-path`                | —        | Path to a point cloud file or directory              |
| `--output-path`               | —        | Output `.tar` archive path                           |
| `--source-crs`                | —        | EPSG code of the input coordinate system             |
| `--maximum-points-per-octant` | `100000` | Maximum points per octree node                       |
| `--no-shuffle`                | —        | Disable random shuffling of points before conversion |
| `--seed`                      | `1`      | Seed for reproducible shuffling                      |

### Supported input formats

| Format          | Extension  |
|-----------------|------------|
| LAS             | `.las`     |
| LAZ             | `.laz`     |
| E57             | `.e57`     |
| XYZ             | `.xyz`     |
| XYZ + Zstandard | `.xyz.zst` |

### Output format

Both the CLI and library produce a TAR archive containing:

- `tileset.json` — root 3D Tiles 1.1 document with implicit tiling metadata
- `content/content_{level}_{x}_{y}_{z}.glb` — binary glTF tiles with point positions and colors
- `subtrees/{level}.{x}.{y}.{z}.subtree` — implicit tiling subtree availability metadata

The single-file archive simplifies transfer and can be extracted on the target machine.

---

## etiles (library)

A Rust library for integrating 3D Tiles generation into your own application.

### Installation

```toml
[dependencies]
etiles = "0.0.2-alpha.1" # replace with the latest version
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
