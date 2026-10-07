# SYN Studio

**SYN** is an experimental creative document format and authoring environment.

The goal is bigger than a file container: one shareable SYN document should be able to describe an interactive experience such as a birthday card, story, music experience, presentation, portfolio, website, or eventually a game.

## Prototype

The first Studio prototype lives in [studio/index.html](./studio/index.html).

It currently demonstrates:

- a visual canvas
- text, media, shape, and button tools
- object selection and inspection
- event → action interactions
- direct `.syn` export
- a deliberately inspectable JSON-based 0.1 document model

## Architecture

- **SYN Studio**: visual authoring environment
- **SYN Format**: portable creative document model
- **SYN Runtime**: future sandboxed renderer/player
- **SYN AI**: future native authoring assistant that edits the SYN model itself

## Security direction

The core rule is simple:

> **A SYN file is data, not authority.**

The runtime should sandbox documents, use explicit capabilities, avoid unrestricted filesystem/shell access, and keep network access opt-in. General-purpose executable code is not part of the 0.1 core model.

## Status

This is an experimental project. The 0.1 format is intentionally not frozen yet.

See [spec/FORMAT.md](./spec/FORMAT.md) for the current format principles.
