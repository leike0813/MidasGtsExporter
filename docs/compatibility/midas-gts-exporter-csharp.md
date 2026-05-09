# Original C# Compatibility Notes

## Purpose

This document records behavior from the original C# WPF `MidasGtsExporter` that the Rust/Tauri version must preserve in the first version.

Some behavior is known to be imperfect. The first version preserves it so that users can compare outputs and iterate from a stable compatibility baseline.

## Reference Project

Reference root:

- `references/MidasGtsExporter`

Primary source files:

- `MidasGtsExporter/MidasGtsExporter/GtsFpnDataReader.cs`
- `MidasGtsExporter/MidasGtsExporter/FLAC3D/Flac3dExporter.cs`
- `MidasGtsExporter/MidasGtsExporter/ABAQUS/AbaqusExporter.cs`
- `MidasGtsExporter/MidasGtsExporter/LSDYNA/LsDynaExporter.cs`
- `MidasGtsExporter/MidasGtsExporter/MainWindow.xaml.cs`

## Implemented Formats

The original project implements:

- FLAC3D
- Abaqus
- LS-DYNA

The original `OutputFormat` enum includes `Ansys`, but no UI path or exporter implementation exists. The Rust/Tauri first version must not implement Ansys.

## Parser Compatibility

The original parser:

- reads the entire input file before parsing.
- collects keyword blocks before building the mesh model.
- treats continuation lines as lines whose trimmed form starts with `,`.
- ignores default group ID `1`.
- ignores empty element groups.
- classifies an element group by checking the first element ID in that group.
- does not detect mixed-dimension element groups.
- includes node groups only when the node group ID also exists in `ElemGroups`.

The node-group condition appears inconsistent with the C# comment, but the Rust/Tauri first version must preserve the code behavior.

## FLAC3D Compatibility

The original FLAC3D exporter:

- writes `<output>_Mesh.f3grid`.
- writes `<output>_Geom.geom` only when 1D element groups exist.
- writes only nodes referenced by 3D and 2D elements into the mesh file.
- writes 1D geometry nodes based on 1D elements.
- exposes `CheckInputData`, but the check is effectively not implemented.

The Rust/Tauri first version must preserve these behaviors.

## Abaqus Compatibility

The original Abaqus exporter:

- writes `<output>.inp`.
- writes all nodes.
- converts all 3D elements under `C3D8R`, including lower-node-count elements by repeating node IDs.
- converts 2D elements under `S4R`, including triangles by repeating node IDs.
- converts 1D elements under `B31`.
- writes node groups and element groups.

Known quirk:

- The C# class exposes an unused `LsDynaOption.Option` property.
- The C# statistics block reports the 1D count using `elem2DCount`.

The Rust/Tauri first version does not need to reproduce unused type leakage in the public API, but it must preserve output behavior. Statistics may preserve the displayed messages unless a later compatibility decision says otherwise.

## LS-DYNA Compatibility

The original LS-DYNA exporter:

- writes `<output>.k` only if it does not exist or overwrite is enabled.
- writes include files directly into the selected output folder.
- has an unused include-folder creation helper.
- ignores 1D elements.
- separates 2D groups into shell groups and segment groups.
- treats generated 2D group names starting with `eset_segment_` as segment groups.
- exposes `CheckInputData`, but the check is effectively not implemented.

The Rust/Tauri first version must preserve these behaviors.

## GUI Compatibility

The original GUI:

- defaults to FLAC3D.
- selects a `.fpn` input file.
- defaults the output folder to the selected input file folder.
- defaults the output file name to the input file stem when same-file-name behavior is enabled.
- disables conversion while a background conversion is running.
- displays progress, logs, and elapsed time.

The Rust/Tauri GUI should preserve this workflow while using Tauri-native dialogs and command events.

## Known Deferred Improvements

These are intentionally deferred:

- streaming parser as an externally visible compatibility topic.
- robust text encoding detection.
- validation of duplicate IDs before insertion.
- validation of missing node references.
- validation of mixed-dimension groups.
- sanitization of output group names.
- correction of original statistics display quirks.
- CLI support as a first-class interface.
