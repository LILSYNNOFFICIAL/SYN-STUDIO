# SYN Studio Authoring Environment Implementation Plan

> **For agentic workers:** Use the host's available task-by-task implementation workflow. Steps use checkbox syntax for tracking.

**Goal:** Turn SYN Studio into a polished visual-plus-code creative authoring environment with a deep professional menu system and an impressive editable first-run experience.

**Architecture:** Preserve the vanilla browser ES-module architecture. Extend the SYN document model with structured styles, links, source metadata, and safe declarative actions. Build the UI around a data-driven command/menu registry, contextual inspector, code/source workspace, and real demo document.

**Tech Stack:** HTML, CSS, browser ES modules, Node.js 22, node:test, GitHub Actions, GitHub Pages.

## Global Constraints
- SYN remains a creative document, not a generic archive or executable container.
- A SYN file is data, not authority.
- No arbitrary native code, shell, filesystem, or unrestricted network execution.
- JavaScript is the scripting language. Do not invent a general-purpose language.
- Visual and code views operate on the same document model.
- Database credentials and secrets are never embedded in SYN documents.
- Preserve existing import/export/runtime behavior and tests.
- Keep the first-run Studio state populated with a real editable demo.

## Task 1: Expand the SYN model
Files: src/document.js, src/runtime.js, test/document.test.js, test/runtime.test.js.

1. Add failing tests for structured styles, safe URL/SYN links, source metadata, and unsafe action/media rejection.
2. Run focused node:test commands and confirm failures.
3. Add optional styles, links, and source properties while preserving old 0.1 documents. Add safe link.openUrl and link.openSyn actions with validation. Keep remote media blocked.
4. Run npm test and confirm the complete suite passes.
5. Commit as feat: expand syn document authoring model.

## Task 2: Professional Studio shell
Files: studio/index.html, studio/styles.css, studio/app.js, test/syntax.test.js.

1. Add a data-driven menu/command registry with deep categories including File, Edit, View, Insert, Format, Arrange, Object, Project, Scene, Timeline, Animation, Media, Audio, Video, Typography, Components, Symbols, Layout, Responsive, Interaction, Navigation, Logic, State, Signals, Variables, Data, Database, API, Forms, Web, Code, Debug, AI, Assets, Effects, Accessibility, Localization, Security, Performance, Version, Collaboration, Build, Package, Publish, Tools, Window, Help, and Experimental.
2. Keep common actions in a compact toolbar. Add Ctrl/Cmd+K command palette.
3. Add polished graphite/neutral controls and object rendering. Add direct double-click text editing with Enter/Escape.
4. Expand the inspector for typography, colors, appearance, geometry, links, and metadata.
5. Run npm test and browser syntax checks.
6. Commit as feat: build professional studio shell.

## Task 3: Code workspace, demo, media, navigation
Files: studio/index.html, studio/styles.css, studio/app.js, examples/hello.syn, spec/FORMAT.md, README.md, tests.

1. Add tests for demo complexity, source round-trip, safe navigation, and embedded media.
2. Add Design, Code, Split, Preview, and Debug workspaces. Code view edits the same document model, with raw/generated representation clearly separated from JavaScript source.
3. Add a multi-scene polished first-run demo with typography, styled buttons, media, navigation, and interactions. Add image import and replacement. Add scene, URL, and cross-SYN link properties.
4. Make runtime honor supported style properties and safe links.
5. Run npm test and static asset/syntax smoke checks.
6. Commit as feat: add code workspace and first-run demo.

## Task 4: Runtime and release verification
Files: runtime/index.html, runtime/runtime.js, runtime/runtime.css, sw.js, README.md, tests.

1. Add focused runtime/style/cache-version tests.
2. Update runtime rendering for supported styles and clear errors. Version the service-worker cache with each shell update.
3. Run npm test, syntax checks, and inspect GitHub Actions results after push.
4. Commit as chore: verify syn studio runtime shell.

## Scope boundary
The UI may expose future capability categories, but this pass must not pretend arbitrary JavaScript execution, PHP execution, production database access, multiplayer, 3D, or physics are already implemented. Those remain extension points behind the capability/security architecture.
