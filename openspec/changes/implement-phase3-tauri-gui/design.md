# Design: Phase 3 Tauri GUI

## Architecture

Phase 3 connects the existing Rust parser/exporter backend to a React + TypeScript Tauri GUI.

The backend owns:

- request validation.
- parsing `.fpn`.
- dispatching exporters.
- progress event emission.
- structured error mapping.

The frontend owns:

- file/folder selection.
- user-editable conversion options.
- invoking `convert_mesh`.
- displaying progress, logs, elapsed time, and output files.

The frontend must not reimplement parser or exporter logic.

## Backend Command

Add a Tauri command conceptually matching:

```rust
#[tauri::command]
async fn convert_mesh(
    app: tauri::AppHandle,
    request: ConvertRequest,
) -> Result<ConvertResponse, ConvertError>
```

Command flow:

1. Validate `input_file_path`, `output_folder_path`, `output_file_name`, and `output_format`.
2. Emit start/progress log events.
3. Parse the input `.fpn` file using the Phase 1 parser.
4. Dispatch to the selected Phase 2 exporter.
5. Emit output/statistics/completion progress events.
6. Return written output file paths and elapsed milliseconds.

The command should use a blocking worker or equivalent if needed so the UI does not freeze during large conversions.

## Frontend Workflow

The GUI should preserve the original WPF workflow:

- FLAC3D selected by default.
- Input file button opens a `.fpn` file dialog.
- Output folder defaults to the selected input file's folder.
- Output file name defaults to the selected input file stem when same-file-name behavior is enabled.
- Output filename can be edited when same-file-name behavior is disabled.
- Format-specific options are shown only for the selected format.
- Convert button is disabled while conversion is running.
- Progress bar and log panel update from backend progress events.
- Elapsed time is shown while conversion is running and after completion.

## Tauri Plugins and Permissions

Use official Tauri v2 dialog plugin support:

- Rust dependency: `tauri-plugin-dialog`.
- Frontend dependency: `@tauri-apps/plugin-dialog`.
- Initialize the dialog plugin in Tauri app setup.
- Add dialog open permission to the default capability.

The frontend should use the dialog plugin for:

- selecting `.fpn` files.
- selecting output folders.

## Request and Error Types

Request, response, progress, and error structures should follow `docs/contracts/tauri-command-contract.md`.

Output formats:

- `flac3d`
- `abaqus`
- `lsdyna`

Error codes should distinguish at least:

- invalid request.
- input file not found.
- output folder not found.
- read failure.
- parse failure.
- write failure.
- unsupported format.
- internal error.

## Testing and Validation

Backend tests should cover:

- valid request dispatch for each supported format using temporary directories.
- invalid request handling.
- conversion response output file list.
- parser/exporter error mapping where practical.

Frontend validation should cover:

- TypeScript compile through `npm run build`.
- request construction and default state through type-level or lightweight unit checks where practical.

Tauri native build may still be blocked by Linux WebKit system packages. If so, record the environment blocker and keep parser/exporter/frontend build validation as the correctness gate.
