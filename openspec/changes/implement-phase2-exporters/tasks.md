# Tasks

- [x] Add Rust backend exporter module layout for FLAC3D, Abaqus, LS-DYNA, and shared helpers.
- [x] Add shared exporter options, output file list, and structured export/write errors.
- [x] Implement group-name helpers using original `nset_` and `eset_` prefixes.
- [x] Implement shared ID slicing and element node-order mapping helpers.
- [x] Implement FLAC3D mesh writer for nodes, zones, faces, and groups.
- [x] Implement FLAC3D geometry writer for 1D nodes, edges, and geometry groups.
- [x] Implement Abaqus writer for all nodes, 3D/2D/1D elements, node groups, and element groups.
- [x] Implement LS-DYNA main file writer with `overwrite_main_output_file` behavior.
- [x] Implement LS-DYNA part, node, element, node-list, and segment include writers.
- [x] Preserve C# compatibility rules documented in `docs/contracts/exporter-contract.md`.
- [x] Add exporter tests using reference `.fpn` fixtures and temporary output directories.
- [x] Add tests for key output files, nonempty content, required section headers, and node-order mapping helpers.
- [x] Add LS-DYNA overwrite behavior tests.
- [x] Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] Run `npm run build`.
- [x] Run `openspec validate implement-phase2-exporters`.

## Non-Tasks

- Do not implement the full GUI conversion workflow in this change.
- Do not implement Tauri command wiring for user conversion in this change.
- Do not implement Ansys support.
- Do not fix documented C# compatibility quirks.
- Do not add byte-for-byte golden output tests unless original C# output files are supplied.
- Do not start a development server.
