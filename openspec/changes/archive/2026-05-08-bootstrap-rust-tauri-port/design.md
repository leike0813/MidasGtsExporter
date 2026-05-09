# Design: Documentation Baseline

## Design Principles

- Preserve original behavior first.
- Separate parser, mesh model, exporters, and GUI IPC.
- Keep GUI business-light and backend business-heavy.
- Record legacy quirks rather than silently fixing them.
- Defer behavior changes until real user files or explicit decisions justify them.

## Document Set

The documentation baseline consists of:

- `docs/roadmap.md`
- `docs/contracts/fpn-parser-contract.md`
- `docs/contracts/exporter-contract.md`
- `docs/contracts/tauri-command-contract.md`
- `docs/compatibility/midas-gts-exporter-csharp.md`

## Contract Boundaries

### FPN Parser

Owns:

- reading `.fpn` records.
- handling continuation lines.
- parsing nodes and elements.
- parsing group names.
- classifying element groups.
- producing deterministic internal mesh data.

Does not own:

- target-format node-order mapping.
- output file naming.
- GUI progress rendering.

### Exporters

Own:

- target-format output content.
- output file naming.
- node-order mapping.
- target-format group formatting.
- output statistics messages.

Do not own:

- `.fpn` parsing.
- GUI state.
- file/folder selection.

### Tauri Command Layer

Owns:

- request validation at the command boundary.
- invoking parser and selected exporter.
- returning structured success or error responses.
- emitting progress events.

Does not own:

- target-format content details.
- frontend layout.

### GUI

Owns:

- user input collection.
- option controls.
- progress and elapsed-time display.
- conversion log display.

Does not own:

- parsing.
- exporting.
- mesh transformation.

## Future Implementation Shape

Expected Rust modules:

- `model`
- `fpn_reader`
- `exporters::flac3d`
- `exporters::abaqus`
- `exporters::lsdyna`
- `commands`
- `options`
- `progress`

This design is advisory for later implementation. The current change only creates documentation.
