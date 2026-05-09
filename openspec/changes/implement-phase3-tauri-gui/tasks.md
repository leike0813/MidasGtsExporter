# Tasks

- [x] Add Tauri dialog plugin Rust dependency and initialize it in app setup.
- [x] Add frontend `@tauri-apps/plugin-dialog` dependency.
- [x] Update Tauri capability permissions for dialog open access.
- [x] Add backend conversion DTOs for request, response, progress event, and structured errors.
- [x] Implement `convert_mesh` command with request validation.
- [x] Wire `convert_mesh` to Phase 1 parser and Phase 2 exporters.
- [x] Emit `convert-progress` events for start, parse, export, statistics, and completion stages.
- [x] Map parser/exporter failures into `ConvertError` codes.
- [x] Replace placeholder React shell with WPF-equivalent conversion UI.
- [x] Implement input `.fpn` file picker and output folder picker.
- [x] Implement same-file-name output stem behavior.
- [x] Implement output format selector and format-specific options.
- [x] Implement conversion log, progress bar, elapsed time, and output file display.
- [x] Disable conversion controls while conversion is running.
- [x] Add backend tests for valid dispatch, invalid requests, output file lists, and error mapping.
- [x] Add frontend type/build validation for request construction and default state where practical.
- [x] Run `cargo test --manifest-path src-tauri/Cargo.toml`.
- [x] Run `npm run build`.
- [x] Attempt `npm run tauri -- build --debug --features app` and record any local WebKit dependency blocker.
- [x] Run `openspec validate implement-phase3-tauri-gui`.

## Non-Tasks

- Do not implement Ansys support.
- Do not change parser compatibility behavior.
- Do not change exporter compatibility behavior.
- Do not implement Phase 4 real-file comparison.
- Do not start a development server.
