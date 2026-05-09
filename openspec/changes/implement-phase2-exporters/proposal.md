# Change: Implement Phase 2 Exporters

## Why

Phase 1 established a Rust/Tauri skeleton and a tested `.fpn` parser core. The next roadmap phase is to reproduce the original C# project's implemented exporters so parsed Midas GTS models can be written to FLAC3D, Abaqus, and LS-DYNA mesh formats.

The repository does not currently contain original C# generated output files for byte-for-byte golden comparison. Phase 2 therefore uses contract-driven output checks and focused node-order mapping tests until real golden files are available.

## What Changes

Implement exporter support for the original project's three implemented output formats:

- FLAC3D
- Abaqus
- LS-DYNA

The implementation must reuse the Phase 1 parser and mesh model, preserve output behavior documented in `docs/contracts/exporter-contract.md`, and keep known C# compatibility quirks unless a later change explicitly revises them.

## Scope

In scope:

- Add exporter modules and shared writer helpers to the Rust backend.
- Add format-specific option structs.
- Generate FLAC3D output files:
  - `<output>_Mesh.f3grid`
  - `<output>_Geom.geom` when 1D geometry data exists.
- Generate Abaqus output file:
  - `<output>.inp`
- Generate LS-DYNA output files:
  - `<output>.k`
  - `<output>_Part.k`
  - `<output>_Node.k`
  - `<output>_Elem.k`
  - `<output>_NodeList.k`
  - `<output>_Segment.k`
- Preserve output filenames, grouping behavior, element node-order mappings, and LS-DYNA overwrite behavior.
- Add tests using reference `.fpn` fixtures and temporary output directories.

Out of scope:

- Full GUI conversion workflow.
- Tauri command wiring for user conversion.
- Ansys support.
- Fixing documented C# compatibility quirks.
- Byte-for-byte golden comparison against original C# output files.
- Starting a development server.

## Expected Outcome

After implementation, the Rust backend can parse the reference `.fpn` files and export FLAC3D, Abaqus, and LS-DYNA output files into a selected output directory through backend APIs.
