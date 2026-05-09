## ADDED Requirements

### Requirement: Shared Exporter Backend

The Rust backend SHALL provide exporter APIs that consume the Phase 1 parsed mesh model and write target-format files to a selected output directory.

#### Scenario: Exporters return written output files

- **GIVEN** a parsed mesh model, output folder, and output file stem
- **WHEN** a backend exporter completes successfully
- **THEN** it returns the list of files actually written
- **AND** those files are located under the requested output folder
- **AND** write failures are returned as structured errors rather than panics

### Requirement: FLAC3D Exporter

The Rust backend SHALL export FLAC3D files according to `docs/contracts/exporter-contract.md`.

#### Scenario: FLAC3D mesh file is written

- **GIVEN** a parsed mesh containing 3D or 2D elements
- **WHEN** the FLAC3D exporter runs
- **THEN** it writes `<output>_Mesh.f3grid`
- **AND** the file contains FLAC3D gridpoint, zone or face, and group sections as applicable
- **AND** element node order follows the FLAC3D mapping contract

#### Scenario: FLAC3D geometry file is written only for 1D geometry data

- **GIVEN** a parsed mesh
- **WHEN** the FLAC3D exporter runs
- **THEN** it writes `<output>_Geom.geom` only when 1D elements and 1D groups exist
- **AND** geometry edges and geometry groups follow the FLAC3D contract

### Requirement: Abaqus Exporter

The Rust backend SHALL export Abaqus `.inp` files according to `docs/contracts/exporter-contract.md`.

#### Scenario: Abaqus inp file includes nodes, elements, and groups

- **GIVEN** a parsed mesh
- **WHEN** the Abaqus exporter runs
- **THEN** it writes `<output>.inp`
- **AND** the file includes all parsed nodes
- **AND** the file includes 3D, 2D, and 1D element sections when those elements exist
- **AND** the file includes node groups and element groups when those groups exist
- **AND** lower-node-count element node repetition follows the Abaqus mapping contract

### Requirement: LS-DYNA Exporter

The Rust backend SHALL export LS-DYNA files according to `docs/contracts/exporter-contract.md`.

#### Scenario: LS-DYNA include files are written to the output folder

- **GIVEN** a parsed mesh
- **WHEN** the LS-DYNA exporter runs
- **THEN** it writes include files directly into the selected output folder
- **AND** it writes part, node, element, node-list, and segment files according to available mesh data
- **AND** it ignores 1D elements
- **AND** 2D groups with generated names starting with `eset_segment_` are written as segment sets

#### Scenario: LS-DYNA main file honors overwrite option

- **GIVEN** `<output>.k` already exists
- **WHEN** the LS-DYNA exporter runs with `overwrite_main_output_file` set to false
- **THEN** it does not overwrite the existing main file
- **WHEN** the LS-DYNA exporter runs with `overwrite_main_output_file` set to true
- **THEN** it writes the main file using the LS-DYNA contract

### Requirement: Exporter Contract Tests

The project SHALL include tests that validate exporter behavior against reference `.fpn` fixtures and documented mapping contracts.

#### Scenario: Reference fixtures can be exported to all Phase 2 formats

- **GIVEN** reference `.fpn` fixtures under `references/MidasGtsExporter/TestData/MidasGts`
- **WHEN** exporter tests parse each fixture and export FLAC3D, Abaqus, and LS-DYNA outputs to temporary directories
- **THEN** expected output files exist
- **AND** generated files are nonempty
- **AND** required format headers or sections are present
- **AND** element node-order mapping helpers are covered by unit tests
