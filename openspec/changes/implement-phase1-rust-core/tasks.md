# Tasks

- [x] Create Tauri v2 project skeleton with React + TypeScript and `npm`.
- [x] Add Rust backend module layout for `model`, `fpn_reader`, and parser/core errors.
- [x] Define core mesh data structures for nodes, elements, grouped element maps, grouped node maps, and group-name maps.
- [x] Implement `.fpn` keyword discovery and continuation-line handling.
- [x] Implement node, 3D element, 2D element, 1D element, group-name, element-group, and node-group parsing.
- [x] Preserve C# grouping compatibility rules from `docs/contracts/fpn-parser-contract.md`.
- [x] Add parser tests using `.fpn` fixture files under `tests/fixtures/midas_gts`.
- [x] Verify parser tests assert successful parse, nonzero node count, nonzero element count, deterministic group maps, default group ID exclusion, and continuation-line handling.
- [x] Run `cargo test` for Rust tests.
- [x] Run the available `npm` or Tauri validation command after skeleton creation.
- [x] Run `openspec validate implement-phase1-rust-core`.

## Non-Tasks

- Do not implement FLAC3D exporter in this change.
- Do not implement Abaqus exporter in this change.
- Do not implement LS-DYNA exporter in this change.
- Do not implement Ansys support.
- Do not implement the full GUI conversion workflow.
- Do not revise documented C# compatibility quirks.
