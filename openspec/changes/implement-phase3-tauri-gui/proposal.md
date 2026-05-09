# Change: Implement Phase 3 Tauri GUI

## Why

Phase 1 established the Rust/Tauri skeleton and parser core. Phase 2 added backend exporters for FLAC3D, Abaqus, and LS-DYNA. The next roadmap phase is to expose those backend capabilities through a desktop GUI equivalent to the original C# WPF workflow.

This change defines the GUI and command integration work required for users to complete a conversion from the desktop application without manually calling backend APIs.

## What Changes

Implement the Tauri GUI conversion workflow:

- Add a backend `convert_mesh` command following `docs/contracts/tauri-command-contract.md`.
- Wire parser and exporter APIs behind the command.
- Emit conversion progress events.
- Add structured conversion errors.
- Add React UI for input file selection, output folder selection, output filename, output format, options, progress, elapsed time, and logs.
- Add Tauri dialog plugin support for file and folder selection.

## Scope

In scope:

- Tauri command and event wiring.
- Request/response/error DTOs for conversion.
- Dialog plugin dependencies, initialization, and permissions.
- React GUI equivalent to the original WPF conversion workflow.
- Frontend state for format-specific options.
- Frontend event listening for `convert-progress`.
- Backend and frontend validation tests where practical.

Out of scope:

- Ansys support.
- Parser behavior changes.
- Exporter behavior changes.
- Phase 4 compatibility iteration and real-file comparison.
- Starting a development server as part of implementation.

## Expected Outcome

After implementation, a user can select a `.fpn` file, choose an output folder and format, run conversion, see progress/logs/elapsed time, and receive generated FLAC3D, Abaqus, or LS-DYNA files through the Tauri desktop UI.
