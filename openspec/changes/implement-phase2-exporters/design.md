# Design: Phase 2 Exporters

## Architecture

Phase 2 adds Rust backend exporter modules on top of the Phase 1 parser and mesh model.

Recommended module layout:

- `exporters::mod`: shared exporter types, output file list, and common helpers.
- `exporters::flac3d`: FLAC3D mesh and geometry writer.
- `exporters::abaqus`: Abaqus `.inp` writer.
- `exporters::lsdyna`: LS-DYNA main/include file writers.

The frontend remains a minimal shell in this phase. Exporters are backend-only and are not exposed through a full GUI conversion workflow yet.

## Shared Exporter Model

Exporter APIs should accept:

- parsed `Mesh`.
- output folder path.
- output file name without extension.
- format-specific options.

Exporter APIs should return:

- list of files actually written.
- structured write/export errors.

Shared helpers should cover:

- creating target output paths.
- writing text files.
- formatting scientific notation consistently enough for contract tests.
- slicing ID lists into fixed-size rows.
- generating group names with original prefixes:
  - `nset_`
  - `eset_`

## FLAC3D Exporter

The FLAC3D writer follows `docs/contracts/exporter-contract.md`.

Key behaviors:

- Write mesh output to `<output>_Mesh.f3grid`.
- Write geometry output to `<output>_Geom.geom` only when 1D elements and 1D groups exist.
- Mesh nodes are nodes referenced by 3D and 2D elements.
- 3D groups use `ZGROUP "<group>" SLOT 1`.
- 2D groups use `FGROUP "<group>" SLOT 2`.
- 1D geometry groups use `GROUP "Geom" "<group>"`.
- `check_input_data` remains a compatibility no-op.

## Abaqus Exporter

The Abaqus writer follows `docs/contracts/exporter-contract.md`.

Key behaviors:

- Write output to `<output>.inp`.
- Write all parsed nodes.
- Write 3D elements under `C3D8R`.
- Write 2D elements under `S4R`.
- Write 1D elements under `B31`.
- Repeat node IDs for lower-node-count source elements exactly as documented.
- Write node and element groups with 16 IDs per line.
- Do not expose the unused C# `LsDynaOption.Option` leakage.

## LS-DYNA Exporter

The LS-DYNA writer follows `docs/contracts/exporter-contract.md`.

Key behaviors:

- Write include files directly into the selected output folder.
- Write `<output>.k` only when it does not exist or `overwrite_main_output_file` is true.
- Always write the part, node, element, node-list, and segment files when their corresponding data exists or the original implementation would create them.
- Ignore 1D elements.
- Split 2D groups into shell groups and segment groups.
- Treat generated 2D group names starting with `eset_segment_`, case-insensitive, as segment groups.
- `check_input_data` remains a compatibility no-op.

## Test Strategy

Because no original C# output files are available, tests should be contract-driven:

- Use reference `.fpn` fixtures from `references/MidasGtsExporter/TestData/MidasGts`.
- Export each format into a temporary directory.
- Assert expected files exist and are nonempty.
- Assert key headers and sections exist.
- Unit-test node-order mapping helpers directly.
- Test LS-DYNA overwrite false/true behavior for the main `.k` file.

If original C# output files become available later, add golden comparison through a separate compatibility change.

## Validation

Implementation validation should include:

- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run build`
- `openspec validate implement-phase2-exporters`

Tauri native build remains dependent on local WebKit system packages and is not required for exporter correctness.
