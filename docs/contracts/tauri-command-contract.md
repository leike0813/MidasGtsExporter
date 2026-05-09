# Tauri Command Contract

## Purpose

The Tauri command layer connects the GUI to the Rust conversion core.

Frontend code must not implement parser or exporter business logic. It collects user input, invokes backend commands, and displays progress events.

## Output Format

```ts
type OutputFormat = "flac3d" | "abaqus" | "lsdyna";
```

`ansys` is intentionally absent because the original C# project did not implement an Ansys exporter.

## Convert Request

```ts
interface ConvertRequest {
  inputFilePath: string;
  outputFolderPath: string;
  outputFileName: string;
  outputFormat: OutputFormat;
  flac3d?: Flac3dOptions;
  lsdyna?: LsDynaOptions;
}

interface Flac3dOptions {
  checkInputData: boolean;
}

interface LsDynaOptions {
  checkInputData: boolean;
  overwriteMainOutputFile: boolean;
}
```

Rules:

- `inputFilePath` must point to a user-selected `.fpn` file.
- `outputFolderPath` must point to a user-selected folder.
- `outputFileName` excludes the extension.
- `flac3d` is required when `outputFormat` is `"flac3d"`.
- `lsdyna` is required when `outputFormat` is `"lsdyna"`.
- Abaqus has no first-version options.

## Convert Response

```ts
interface ConvertResponse {
  success: boolean;
  outputFiles: string[];
  elapsedMs: number;
}
```

Rules:

- `success` is true only when the selected exporter completes without error.
- `outputFiles` lists files that were actually written by the backend.
- For LS-DYNA, the main `<output>.k` file is included only when it was written or overwritten.
- `elapsedMs` is measured by the Rust backend.

## Progress Event

```ts
interface ConvertProgressEvent {
  percentage: number;
  message: string;
}
```

Event name:

```ts
const CONVERT_PROGRESS_EVENT = "convert-progress";
```

Rules:

- Events are emitted during conversion.
- `percentage` is an integer in `[0, 100]`.
- `message` is a user-facing log line.
- The GUI appends messages to the conversion log in event order.
- The GUI updates the progress bar with the latest percentage.

## Error Response

Tauri commands should return structured errors.

```ts
interface ConvertError {
  code: ConvertErrorCode;
  message: string;
  detail?: string;
}

type ConvertErrorCode =
  | "invalid-request"
  | "input-file-not-found"
  | "output-folder-not-found"
  | "read-failed"
  | "parse-failed"
  | "write-failed"
  | "unsupported-format"
  | "internal-error";
```

Rules:

- `message` is suitable for display in the GUI.
- `detail` may contain lower-level diagnostics.
- Errors should not panic across the Tauri command boundary.

## Backend Command Shape

The backend command should have this conceptual shape:

```rust
#[tauri::command]
async fn convert_mesh(
    app: tauri::AppHandle,
    request: ConvertRequest,
) -> Result<ConvertResponse, ConvertError>
```

Implementation may use blocking worker threads internally if needed to avoid freezing the GUI.

## GUI Responsibilities

The GUI is responsible for:

- file selection.
- folder selection.
- output format selection.
- output file name editing.
- format-specific option controls.
- elapsed-time display.
- progress-bar display.
- conversion log display.

The GUI is not responsible for:

- parsing `.fpn`.
- classifying elements.
- generating output file content.
- deciding output file names beyond passing `outputFileName`.

## Compatibility Notes

The GUI should preserve the original workflow:

- FLAC3D selected by default.
- Output folder defaults to the input file folder after file selection.
- Output file name defaults to the input file stem when "same file name" behavior is enabled.
- Conversion is disabled while a conversion is running.
