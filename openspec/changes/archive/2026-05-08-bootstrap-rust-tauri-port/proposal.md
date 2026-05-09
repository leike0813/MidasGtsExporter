# Change: Bootstrap Rust/Tauri MidasGtsExporter Port

## Summary

Create the documentation baseline for a Rust + Tauri desktop application that reproduces the implemented behavior of the original C# WPF `MidasGtsExporter`.

This change does not implement application code. It establishes roadmap, parser contract, exporter contract, Tauri command contract, and original-project compatibility notes.

## Motivation

The original C# project contains the conversion knowledge needed for a Rust/Tauri port, but it also contains GUI coupling, repeated exporter logic, and legacy quirks. Before implementation starts, the project needs explicit documentation constraints so that compatibility decisions are deliberate rather than accidental.

## Scope

In scope:

- Roadmap for staged implementation.
- FPN parser contract.
- Exporter contract for FLAC3D, Abaqus, and LS-DYNA.
- Tauri command contract for GUI/backend communication.
- Compatibility notes for original C# behavior.

Out of scope:

- Creating the Rust crate.
- Creating the Tauri app.
- Implementing parser logic.
- Implementing exporters.
- Implementing GUI.
- Fixing original-project behavioral quirks.

## Compatibility Position

The first implementation phase must prioritize one-to-one behavioral compatibility with the original C# project for implemented formats:

- FLAC3D
- Abaqus
- LS-DYNA

No Ansys exporter is included because the original project did not implement one.

## Expected Outcome

After this change, implementation can proceed against stable written contracts without re-litigating basic project boundaries.
