# Exporter Contract

## Purpose

Exporters convert the parsed internal mesh model into files compatible with the original C# `MidasGtsExporter` output behavior.

The first Rust/Tauri version must preserve implemented C# behavior, including known quirks, unless a later change explicitly revises compatibility.

## Shared Inputs

Each exporter receives:

- parsed mesh model.
- input file path.
- output folder path.
- output file name without extension.
- format-specific options.
- progress reporter.

Each exporter must emit progress messages compatible with the GUI contract.

## Shared Naming Rules

Group names follow the original prefixes:

- Node group name: `nset_<original_group_name>`
- Element group name: `eset_<original_group_name>`

The implementation does not sanitize group names in the first version unless required by Rust string handling. This preserves original behavior.

## FLAC3D Exporter

Output files:

- `<output>_Mesh.f3grid`
- `<output>_Geom.geom`

Options:

- `check_input_data: bool`

Compatibility behavior:

- `check_input_data` reports progress but does not perform validation in the first version.
- Mesh output includes nodes referenced by 3D and 2D elements.
- Geometry output is created only when 1D elements and 1D groups exist.

FLAC3D 3D element mappings:

| Source node count | Source kind | Output prefix | Node order |
|---:|---|---|---|
| 8 | HEXA | `Z B8` | `1,2,4,5,3,8,6,7` |
| 6 | PRISM | `Z W6` | `1,3,4,2,6,5` |
| 5 | PYRAM | `Z P5` | `1,2,4,5,3` |
| 4 | TETRA | `Z T4` | `1,2,3,4` |

FLAC3D 2D element mappings:

| Source node count | Source kind | Output prefix | Node order |
|---:|---|---|---|
| 4 | RECT | `F Q4` | `1,2,3,4` |
| 3 | TRIA | `F T3` | `1,2,3` |

FLAC3D groups:

- 3D element groups emit `ZGROUP "<group>" SLOT 1`.
- 2D element groups emit `FGROUP "<group>" SLOT 2`.
- Group IDs are written in slices of 20 IDs per line.

FLAC3D geometry:

- Header:
  - `ITASCA GEOMETRY3D`
  - `;`
- 1D nodes are written under `NODES`.
- 1D edges are written under `EDGES`.
- Each 1D group emits `GROUP "Geom" "<group>"`.

## Abaqus Exporter

Output file:

- `<output>.inp`

Options:

- None in the original implemented GUI path.

Compatibility behavior:

- The original C# exporter contains an unused `LsDynaOption.Option` property. The Rust implementation should not expose this as public API unless needed for compatibility, but behavior must remain option-free from the GUI.
- The first version writes all nodes, not only referenced nodes.
- 3D elements are emitted under `*Element, type=C3D8R, elset=eset_3D`.
- 2D elements are emitted under `*Element, type=S4R, elset=eset_2D`.
- 1D elements are emitted under `*Element, type=B31, elset=eset_1D`.

Abaqus 3D element mappings:

| Source node count | Source kind | Output type line | Node order |
|---:|---|---|---|
| 8 | HEXA | `C3D8R` | `1,2,3,4,5,6,7,8` |
| 6 | PRISM | `C3D8R` | `1,2,3,3,4,5,6,6` |
| 5 | PYRAM | `C3D8R` | `1,2,3,4,5,5,5,5` |
| 4 | TETRA | `C3D8R` | `1,2,3,3,4,4,4,4` |

Abaqus 2D element mappings:

| Source node count | Source kind | Output type line | Node order |
|---:|---|---|---|
| 4 | RECT | `S4R` | `1,2,3,4` |
| 3 | TRIA | `S4R` | `1,2,3,3` |

Abaqus 1D element mappings:

| Source node count | Source kind | Output type line | Node order |
|---:|---|---|---|
| 2 | LINE | `B31` | `1,2` |

Abaqus groups:

- Node groups emit `*Nset, nset=<group>`.
- Element groups emit `*Elset, elset=<group>`.
- Group IDs are written in slices of 16 IDs per line.

## LS-DYNA Exporter

Output files:

- `<output>.k`
- `<output>_Part.k`
- `<output>_Node.k`
- `<output>_Elem.k`
- `<output>_NodeList.k`
- `<output>_Segment.k`

Options:

- `check_input_data: bool`
- `overwrite_main_output_file: bool`

Compatibility behavior:

- `check_input_data` reports progress but does not perform validation in the first version.
- `<output>.k` is written only if it does not exist or `overwrite_main_output_file` is true.
- Include files are written directly into the selected output folder.
- The original `IncludeFolderName` concept is not active because the C# `IncludeFolderPath` returns `OutputFolderPath`.
- 1D elements are ignored by the LS-DYNA exporter, matching the C# implementation.

LS-DYNA segment grouping:

- 2D groups whose generated name starts with `eset_segment_`, case-insensitive, are written as segment sets.
- Other 2D groups are written as shell elements.

LS-DYNA 3D element mappings:

| Source node count | Source kind | Output keyword | Node order |
|---:|---|---|---|
| 8 | HEXA | `*ELEMENT_SOLID` | `1,2,3,4,5,6,7,8` |
| 6 | PRISM | `*ELEMENT_SOLID` | `3,2,5,6,1,1,4,4` |
| 5 | PYRAM | `*ELEMENT_SOLID` | `1,2,3,4,5,5,5,5` |
| 4 | TETRA | `*ELEMENT_SOLID` | `1,2,3,4,4,4,4,4` |

LS-DYNA 2D shell mappings:

| Source node count | Source kind | Output keyword | Node order |
|---:|---|---|---|
| 4 | RECT | `*ELEMENT_SHELL` | `1,2,3,4` |
| 3 | TRIA | `*ELEMENT_SHELL` | `1,2,3,3` |

LS-DYNA segment mappings:

| Source node count | Source kind | Node order |
|---:|---|---|
| 4 | RECT | `1,2,3,4` |
| 3 | TRIA | `1,2,3,3` |

## Progress Reporting

Exporter progress should preserve the broad original stages:

- start conversion.
- parse FPN.
- convert mesh data.
- optional input data check.
- output target-format mesh data.
- output statistics.
- conversion completed.

Exact progress percentages may follow the C# project where practical, but the compatibility requirement is message and stage preservation rather than exact timing.

## Compatibility References

Primary reference files:

- `references/MidasGtsExporter/MidasGtsExporter/MidasGtsExporter/FLAC3D/Flac3dExporter.cs`
- `references/MidasGtsExporter/MidasGtsExporter/MidasGtsExporter/ABAQUS/AbaqusExporter.cs`
- `references/MidasGtsExporter/MidasGtsExporter/MidasGtsExporter/LSDYNA/LsDynaExporter.cs`
