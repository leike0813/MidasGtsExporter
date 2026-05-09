# Change: Implement Phase 1 Rust Core

## Summary

Create the Rust/Tauri project skeleton and implement the first backend core path for the MidasGtsExporter port: mesh data structures, Midas GTS `.fpn` parsing, parser errors, and parser tests.

This phase does not implement exporters or the full GUI conversion workflow.

## Motivation

The project now has a documentation baseline and compatibility contracts. The next step is to establish a compilable Rust/Tauri foundation and prove that reference `.fpn` files can be parsed into the internal mesh model while preserving the original C# parser behavior.

## Scope

In scope:

- Create a Tauri v2 desktop app skeleton.
- Use React + TypeScript for the frontend shell.
- Use `npm` for frontend package management.
- Define Rust core mesh data structures equivalent to the C# `Node`, `Elem`, grouped mesh maps, and group-name maps.
- Implement the `.fpn` parser according to `docs/contracts/fpn-parser-contract.md`.
- Preserve original grouping behavior exactly:
  - ignore default group ID `1`.
  - classify each element group by its first element ID.
  - include node groups only when their group ID exists in `element_groups`.
  - preserve sorted map iteration and parsed group member order.
- Add parser tests using:
  - `references/MidasGtsExporter/TestData/MidasGts/GtsModel_Test.fpn`
  - `references/MidasGtsExporter/TestData/MidasGts/GtsModel_Coarse.fpn`
  - `references/MidasGtsExporter/TestData/MidasGts/GtsModel_Fine.fpn`

Out of scope:

- FLAC3D exporter.
- Abaqus exporter.
- LS-DYNA exporter.
- Full GUI conversion workflow.
- Ansys support.
- Behavior cleanup for known C# quirks.
- Advanced input validation beyond the parser contract.
- Synthetic fixture strategy unless reference fixtures become unusable.

## Compatibility Position

The parser must treat `docs/contracts/fpn-parser-contract.md` and `docs/compatibility/midas-gts-exporter-csharp.md` as source-of-truth documents.

When the original C# implementation has questionable but documented behavior, Phase 1 must preserve that behavior rather than correcting it.

## Expected Outcome

After this change is implemented:

- The repository has a Rust/Tauri project skeleton.
- The Rust backend has a tested `.fpn` parser.
- Parser tests can read the reference `.fpn` files and assert successful parsing.
- The project is ready for Phase 2 exporter implementation.
