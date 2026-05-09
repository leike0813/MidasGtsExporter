## ADDED Requirements

### Requirement: Conversion Command

The Tauri backend SHALL expose a `convert_mesh` command that converts a selected `.fpn` file to one supported output format.

#### Scenario: Valid conversion request completes

- **GIVEN** a valid conversion request with input `.fpn`, output folder, output file stem, and supported output format
- **WHEN** the frontend invokes `convert_mesh`
- **THEN** the backend parses the input file
- **AND** dispatches the selected exporter
- **AND** returns `success: true`, written output files, and elapsed milliseconds

#### Scenario: Invalid conversion request returns structured error

- **GIVEN** a conversion request with a missing input file, missing output folder, empty output stem, or unsupported output format
- **WHEN** the frontend invokes `convert_mesh`
- **THEN** the backend returns a structured `ConvertError`
- **AND** the command does not panic across the Tauri boundary

### Requirement: Conversion Progress Events

The Tauri backend SHALL emit `convert-progress` events during conversion.

#### Scenario: Frontend receives progress logs

- **GIVEN** a conversion is running
- **WHEN** the backend advances through parse and export stages
- **THEN** it emits `convert-progress` events
- **AND** each event includes a percentage in `[0, 100]`
- **AND** each event includes a user-facing message
- **AND** the frontend appends those messages to the conversion log in event order

### Requirement: GUI Conversion Workflow

The React frontend SHALL provide a desktop conversion workflow equivalent to the original WPF application.

#### Scenario: User selects input and output paths

- **GIVEN** the GUI is open
- **WHEN** the user selects a `.fpn` input file
- **THEN** the GUI stores the input path
- **AND** defaults the output folder to the input file's folder
- **AND** defaults the output file name to the input file stem when same-file-name behavior is enabled

#### Scenario: User runs conversion from GUI

- **GIVEN** valid input path, output folder, output file name, and output format are selected
- **WHEN** the user starts conversion
- **THEN** the GUI disables conversion controls while the command is running
- **AND** displays progress, elapsed time, and log messages
- **AND** displays written output files after completion

### Requirement: Format Options UI

The GUI SHALL show only the options relevant to the selected output format.

#### Scenario: FLAC3D options are shown for FLAC3D

- **GIVEN** FLAC3D is selected
- **WHEN** the options panel is rendered
- **THEN** the GUI shows the FLAC3D `checkInputData` option
- **AND** does not show LS-DYNA overwrite options

#### Scenario: LS-DYNA options are shown for LS-DYNA

- **GIVEN** LS-DYNA is selected
- **WHEN** the options panel is rendered
- **THEN** the GUI shows `checkInputData`
- **AND** shows `overwriteMainOutputFile`

#### Scenario: Abaqus has no first-version options

- **GIVEN** Abaqus is selected
- **WHEN** the options panel is rendered
- **THEN** the GUI does not require format-specific options

### Requirement: Dialog Plugin Integration

The Tauri app SHALL use the official Tauri v2 dialog plugin for file and folder selection.

#### Scenario: File and folder dialogs are available

- **GIVEN** the app is running with default capabilities
- **WHEN** the user chooses the input file or output folder buttons
- **THEN** the frontend opens the corresponding Tauri dialog
- **AND** the selected path is written into GUI state
