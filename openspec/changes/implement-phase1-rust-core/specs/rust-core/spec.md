## ADDED Requirements

### Requirement: Tauri Rust Core Skeleton

The project SHALL provide a Tauri v2 application skeleton with a React + TypeScript frontend shell and a Rust backend suitable for parser implementation.

#### Scenario: Skeleton separates frontend and backend responsibilities

- **GIVEN** the Phase 1 implementation is complete
- **WHEN** a developer inspects the project structure
- **THEN** the frontend shell uses React + TypeScript with `npm`
- **AND** the Rust backend contains parser and mesh-model code
- **AND** the frontend does not contain `.fpn` parsing or exporter business logic

### Requirement: Core Mesh Model

The Rust backend SHALL define a core mesh model equivalent to the original C# parser output.

#### Scenario: Parsed mesh exposes deterministic maps

- **GIVEN** a valid Midas GTS `.fpn` file
- **WHEN** the parser produces a mesh model
- **THEN** nodes are available by node ID
- **AND** 3D, 2D, and 1D elements are available by element ID
- **AND** element groups, node groups, and group-name maps are available
- **AND** ID maps iterate deterministically in ascending ID order
- **AND** group member order is preserved as parsed

### Requirement: FPN Parser Compatibility

The Rust backend SHALL parse Midas GTS `.fpn` files according to `docs/contracts/fpn-parser-contract.md`.

#### Scenario: Parser preserves original C# grouping behavior

- **GIVEN** a valid Midas GTS `.fpn` file containing `MSET`, `MSETE`, and `MSETN` records
- **WHEN** the parser builds mesh groups
- **THEN** default group ID `1` is ignored
- **AND** each element group is classified by the first element ID in that group
- **AND** mixed-dimension element groups are not split
- **AND** node groups are included only when their group ID exists in `element_groups`

#### Scenario: Parser handles supported record keywords and continuation lines

- **GIVEN** a valid Midas GTS `.fpn` file containing supported records
- **WHEN** the parser reads the file
- **THEN** it recognizes `NODE`, `HEXA`, `PRISM`, `PYRAM`, `TETRA`, `RECT`, `TRIA`, `LINE`, `MSET`, `MSETE`, and `MSETN`
- **AND** it appends continuation-line fields when the trimmed continuation line starts with `,`
- **AND** it ignores unsupported records without failing the parse

### Requirement: Parser Error Reporting

The Rust backend SHALL report parser failures through structured errors rather than panics.

#### Scenario: Malformed input returns a parser error

- **GIVEN** an input `.fpn` file with malformed numeric fields or malformed record shape
- **WHEN** the parser reads the file
- **THEN** the parser returns a structured error
- **AND** the process does not panic across the public parser API

### Requirement: Reference Fixture Parser Tests

The project SHALL include parser tests using `.fpn` fixture files under `tests/fixtures/midas_gts`.

#### Scenario: Reference fixtures parse successfully

- **GIVEN** `GtsModel_Test.fpn`, `GtsModel_Coarse.fpn`, and `GtsModel_Fine.fpn`
- **WHEN** the parser tests run
- **THEN** each file parses successfully
- **AND** each parsed mesh has a nonzero node count
- **AND** each parsed mesh has a nonzero total element count
- **AND** default group ID `1` is absent from parsed output groups
