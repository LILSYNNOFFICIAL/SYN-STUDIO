# SYN Studio

**SYN** is an experimental creative document format and visual authoring environment.

The goal is bigger than a file container: one shareable SYN document should be able to describe an interactive experience such as a birthday card, story, music experience, presentation, portfolio, website, or eventually a game.

## Current prototype

Open [SYN Studio](./studio/) to use the visual prototype.

It currently demonstrates a visual canvas, text/media/shape/button tools, scene creation, object selection, direct `.syn` export, declarative event → action interactions, and a browser runtime for opening exported `.syn` files.

The core document model in `src/document.js` now supports documents, scenes, objects, interactions, serialization, parsing, and structural validation.

## Architecture

- **SYN Studio**: visual authoring environment
- **SYN Format**: portable creative document model
- **SYN Runtime**: sandboxed renderer/player prototype
- **SYN AI**: future native authoring assistant that edits the SYN model itself

## Security direction

> **A SYN file is data, not authority.**

The runtime should sandbox documents, use explicit capabilities, avoid unrestricted filesystem/shell access, and keep network access opt-in. General-purpose executable code is not part of the 0.1 core model.

## Status

Experimental 0.2 development. The format is intentionally not frozen.

See [spec/FORMAT.md](./spec/FORMAT.md) for the format principles.
