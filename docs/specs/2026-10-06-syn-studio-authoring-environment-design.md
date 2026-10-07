# SYN Studio Authoring Environment Design

## Goal
Expand the prototype into a professional creative authoring environment while preserving a simple visual entry point. The live Studio should open into a polished, complex example experience rather than an empty canvas.

## UX
- Traditional desktop-style horizontal application menu with deep, categorized menus.
- Compact contextual toolbar for frequent actions.
- Premium dark creative-software visual language, restrained accent color, polished controls.
- Canvas supports direct manipulation and double-click text editing.
- Inspector is contextual and exposes typography, color, geometry, states, links, media, interactions, and other properties.
- Code workspace supports Visual, Code, Split, Preview, and Debug views.
- Command palette provides fast access to the large capability surface.

## Menu capability surface
Top-level menus include File, Edit, View, Insert, Format, Arrange, Object, Project, Scene, Timeline, Animation, Media, Audio, Video, Typography, Components, Symbols, Layout, Responsive, Interaction, Navigation, Logic, State, Signals, Variables, Data, Database, API, Forms, Web, Code, Debug, AI, Assets, Effects, Accessibility, Localization, Security, Performance, Version, Collaboration, Build, Package, Publish, Tools, Window, and Help, with Experimental/Labs for future capabilities.

Menus are capability organization, not a requirement that every capability be implemented in this release. The architecture should leave room for the broader model.

## Document architecture
Evolve the SYN document model with stable first-class concepts for:
- documents/projects
- scenes and scene graph hierarchy
- objects and object identities
- styles and typography
- media/assets
- links/navigation
- components/symbols
- declarative interactions
- variables/state/signals
- timelines/animation metadata
- scripts/code references
- capabilities/permissions

The current JSON representation remains inspectable and deterministic. Do not turn the identity of SYN into a generic archive or executable blob.

## Code model
Visual and code authoring are two views over the same document model. Code changes must update the document model and visual changes must update source representation where practical.

Do not create a new general-purpose programming language. JavaScript is the scripting language. SYN source is a structured authoring representation for document structure and declarative behavior, with JavaScript used for computational logic.

The initial implementation may provide a source inspector/editor for the document model and a JavaScript panel without attempting a full compiler. Code execution remains sandboxed and capability-gated.

## Security
- A SYN file is data, not authority.
- No arbitrary native code.
- No unrestricted filesystem or shell access.
- Network and sensitive APIs are capability-gated.
- Database credentials/secrets are never embedded in SYN documents.
- Runtime validates structure, references, actions, and capabilities.

## Live first-run experience
The Studio landing state will contain a polished multi-scene example built from real SYN objects, including typography, buttons, media, navigation, scene links, and interactions. It should demonstrate what the medium can become and be editable by the user.

## Initial implementation boundary
This pass focuses on the authoring foundation: menu system, polished object styling, richer inspector, typography controls, direct text editing, image/media import, scene/document links, code/source workspace, command palette, demo document, and expanded document/runtime model needed to support these features. Full production JavaScript runtime, database backends, multiplayer, 3D, physics, and other advanced capabilities remain architected extension points rather than pretending to be complete now.

## Verification
- Node tests cover document serialization/parsing and runtime validation.
- Browser-facing modules pass syntax checks.
- New model behavior receives tests before or alongside implementation.
- GitHub Actions test workflow must pass.
- GitHub Pages deployment must succeed after changes.
