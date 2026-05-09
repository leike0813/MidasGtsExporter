# MidasGtsExporter Rust/Tauri Port Roadmap

## Project Goal

Build a Rust + Tauri desktop application that reproduces the implemented behavior of the original C# WPF `MidasGtsExporter`.

The first version prioritizes compatibility with the original project over cleanup. Known quirks are documented and preserved unless a later change explicitly revises them.

## Phase 0: Documentation Baseline

Status: completed

Scope:

- Establish the project roadmap.
- Define parser, exporter, and Tauri IPC contracts.
- Record original C# compatibility behavior.
- Define first-version non-goals.

Acceptance criteria:

- `docs/roadmap.md` exists.
- Parser, exporter, and Tauri command contracts exist.
- Original-project compatibility notes exist.
- Implementation can proceed without guessing module boundaries.

## Phase 1: Rust Core

Status: completed

Scope:

- Create the Rust crate and Tauri project skeleton.
- Define core mesh data structures.
- Implement Midas GTS `.fpn` parser.
- Preserve the original grouping behavior.
- Add parser tests using reference `.fpn` fixtures.

Acceptance criteria:

- `.fpn` records can be parsed into nodes, 3D elements, 2D elements, 1D elements, element groups, node groups, and group names.
- Parser behavior matches the documented C# compatibility contract.
- Unsupported or malformed input failures are reported as structured errors.

## Phase 2: Exporters

Status: completed

Scope:

- Implement the FLAC3D exporter.
- Implement the Abaqus exporter.
- Implement the LS-DYNA exporter.
- Preserve original output file names and layout.
- Preserve original element node-order mappings.

Acceptance criteria:

- FLAC3D output files are generated:
  - `<output>_Mesh.f3grid`
  - `<output>_Geom.geom`
- Abaqus output file is generated:
  - `<output>.inp`
- LS-DYNA output files are generated:
  - `<output>.k`
  - `<output>_Part.k`
  - `<output>_Node.k`
  - `<output>_Elem.k`
  - `<output>_NodeList.k`
  - `<output>_Segment.k`
- The Ansys enum from the C# project remains out of scope because no Ansys exporter was implemented.

## Phase 3: Tauri GUI

Status: completed

Scope:

- Build a Tauri desktop GUI equivalent to the original WPF workflow.
- Select the input `.fpn` file.
- Select the output folder.
- Choose the output file name.
- Choose output format.
- Display format-specific options.
- Display progress, logs, and elapsed time.

Acceptance criteria:

- A user can complete conversion entirely from the GUI.
- The GUI calls Rust backend logic through the documented Tauri command contract.
- The GUI does not duplicate parser or exporter business logic.

## Phase 4: Compatibility Iteration

Status: not started

Scope:

- Compare output using real user files when available.
- Add regression fixtures when files become available.
- Separate intentional legacy-compatible behavior from actual Rust-port defects.
- Decide later whether to revise documented legacy quirks.

Acceptance criteria:

- Real-world incompatibilities are tracked as issues or OpenSpec changes.
- Any behavior change from the C# project is backed by an explicit compatibility decision.

## First-Version Non-Goals

- No Ansys exporter.
- No proactive correction of original-project quirks.
- No advanced validation beyond the original behavior.
- No command-line-only release as the primary target.
- No automatic repair of malformed `.fpn` files.
- No format support beyond FLAC3D, Abaqus, and LS-DYNA.
