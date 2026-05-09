# FPN Parser Contract

## Purpose

The FPN parser converts a Midas GTS NX `.fpn` neutral file into the internal mesh model used by all exporters.

The Rust parser must preserve the original C# parser behavior unless a later compatibility change explicitly overrides it.

## Input

Input is a Midas GTS NX `.fpn` file path selected by the GUI.

The first implementation assumes UTF-8 text input. If real files expose encoding incompatibilities, encoding handling must be addressed through a later compatibility change.

## Recognized Records

The parser recognizes these record keywords:

- `NODE`
- `HEXA`
- `PRISM`
- `PYRAM`
- `TETRA`
- `RECT`
- `TRIA`
- `LINE`
- `MSET`
- `MSETE`
- `MSETN`

Record matching follows the original behavior:

- Leading and trailing whitespace is trimmed.
- Keyword matching is case-insensitive during keyword discovery.
- Parsed keyword names are trimmed before classification.
- Continuation lines are lines whose trimmed form starts with `,`.
- Continuation-line fields are appended after skipping the first empty comma field.

## Parsed Model

The parser returns a mesh model with:

- `nodes`: sorted mapping from node ID to node.
- `elements_3d`: sorted mapping from element ID to 3D element.
- `elements_2d`: sorted mapping from element ID to 2D element.
- `elements_1d`: sorted mapping from element ID to 1D element.
- `element_groups`: sorted mapping from group ID to element ID list.
- `element_3d_groups`: sorted mapping from group ID to element ID list.
- `element_2d_groups`: sorted mapping from group ID to element ID list.
- `element_1d_groups`: sorted mapping from group ID to element ID list.
- `node_groups`: sorted mapping from group ID to node ID list.
- `element_group_names`: mapping from group ID to group name.
- `node_group_names`: mapping from group ID to group name.

## Node Parsing

`NODE` records parse:

- field 0: node ID
- field 1: X
- field 2: Y
- field 3: Z

Additional fields are ignored.

## Element Parsing

3D elements:

- `HEXA`: 8 node IDs
- `PRISM`: 6 node IDs
- `PYRAM`: 5 node IDs
- `TETRA`: 4 node IDs

2D elements:

- `RECT`: 4 node IDs
- `TRIA`: 3 node IDs

1D elements:

- `LINE`: 2 node IDs

Element records preserve the original field offset:

- field 0: element ID
- field 1: ignored metadata field
- fields from index 2: node IDs

## Group Parsing

`MSET` records define group names:

- field 0: group ID
- field 1: group name

`MSETE` records define element groups:

- field 0: group ID
- field 1: item count
- fields from index 8: element IDs
- only `item count` IDs are consumed

`MSETN` records define node groups:

- field 0: group ID
- field 1: item count
- fields from index 8: node IDs
- only `item count` IDs are consumed

## Original Grouping Behavior

The default mesh group is ignored:

- group ID `1` is not included in parsed output groups.

Empty element groups are ignored.

Element groups are separated into 3D, 2D, and 1D groups by checking the first element ID in the group:

- If the first element ID exists in `elements_3d`, the entire group is treated as a 3D group.
- Else if it exists in `elements_2d`, the entire group is treated as a 2D group.
- Else if it exists in `elements_1d`, the entire group is treated as a 1D group.

This preserves the C# behavior. Mixed-dimension element groups are not detected or split in the first version.

Node groups are included only when:

- group ID is greater than `1`.
- the group ID exists in `element_groups`.
- the group contains at least one node ID.

This condition intentionally preserves the original code behavior even though its C# comment appears inconsistent.

## Error Handling

The parser should return structured errors for:

- missing input file.
- unreadable input file.
- malformed numeric fields.
- duplicate IDs where the original sorted mapping would reject insertion.
- group names missing for groups that require names.

The first version does not attempt to repair malformed records.

## Ordering

The parser must preserve deterministic output:

- IDs are stored and iterated in ascending order where the C# project used `SortedList`.
- Group ID iteration is ascending.
- ID order inside each group is preserved as parsed.

## Compatibility References

Primary reference files:

- `references/MidasGtsExporter/MidasGtsExporter/MidasGtsExporter/GtsFpnDataReader.cs`
- `references/MidasGtsExporter/MidasGtsExporter/MidasGtsExporter/FemPrimetive.cs`
