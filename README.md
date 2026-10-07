# SYN Studio

**SYN** is an experimental creative document format and visual authoring environment.

The idea is bigger than a file container. A SYN document is meant to be a **shareable creative object** that can contain structure, media, interactions, and eventually richer experiences in one portable format.

The long-term goal is to make creating and sharing things like interactive birthday cards, stories, music experiences, presentations, portfolios, websites, and eventually games as approachable as the creative tools of the Flash era, but built for the modern web with sandboxing and explicit permissions.

## 🚀 Try SYN Studio

**Live:** https://lilsynnofficial.github.io/SYN-STUDIO/

**Source:** https://github.com/LILSYNNOFFICIAL/SYN-STUDIO

The GitHub Pages build is automatically deployed from the `main` branch.

## Current prototype

The current 0.3 prototype demonstrates:

- Visual canvas authoring
- Text, media, shape, and button objects
- Scene creation and navigation
- Object selection, positioning, resizing, and editing
- Declarative event → action interactions
- `.syn` export and import
- A browser-based SYN Runtime
- Embedded image media
- Structural document validation
- Runtime validation of interactions and targets
- Sandboxed, declarative behavior with no arbitrary JavaScript execution from a SYN file
- PWA/offline shell support
- Professional desktop-style menu system with a large developer-oriented capability surface
- Contextual typography and appearance controls
- Double-click text editing on the canvas
- Command palette for discoverability
- Visual, source, and preview workflows
- Multi-scene editable first-run showcase

### Example

The repository includes a working document at [examples/hello.syn](./examples/hello.syn).

It demonstrates a button-driven transition between two scenes and can be opened in the [SYN Runtime](./runtime/).

## Studio authoring model

SYN Studio is intentionally being built as a hybrid creative tool and developer environment. Visual authoring and source inspection operate on the same document model. The current source workspace uses the inspectable SYN JSON representation rather than inventing a replacement programming language. JavaScript is reserved for future sandboxed scripting capabilities.

The top menu exposes a deliberately broad capability architecture covering creative authoring, animation, media, typography, layout, interaction, data, APIs, debugging, AI, security, performance, build, packaging, publishing, and developer tooling. Items that are not implemented yet are clearly treated as extension points rather than simulated features.

## What SYN is trying to become

SYN is being designed as a new kind of creative medium, not simply another archive format.

A future SYN document could combine:

**Media**
- Images
- Audio
- Video
- Typography
- Animation

**Structure**
- Scenes
- Layers
- Components
- Timelines
- Interactive layouts

**Behavior**
- Buttons
- Events
- Transitions
- Media controls
- Declarative logic

**Experiences**
- Digital cards
- Interactive stories
- Music experiences
- Presentations
- Portfolios
- Websites
- Educational experiences
- Games

The important distinction is that the **document itself represents the creative work**, while the runtime provides a controlled environment for experiencing it.

## Architecture

- **SYN Studio**: visual authoring environment
- **SYN Format**: portable creative document model
- **SYN Runtime**: sandboxed renderer/player
- **SYN AI**: planned native authoring assistant that edits the SYN document model directly

The intended workflow is:

`Create → Edit → Preview → Export → Share → Open`

A SYN document should be useful without requiring the recipient to install the original authoring application.

## Security model

> **A SYN file is data, not authority.**

Security is a foundational part of the format rather than something added later.

The direction includes:

- Sandboxed runtime execution
- Declarative behavior instead of arbitrary executable code
- Capability-based permissions
- Explicit permission for sensitive operations
- Network access off by default
- No arbitrary native code
- No unrestricted filesystem or shell access
- Validation of document structure and interaction targets
- Graceful degradation when a capability is unavailable

The core principle is simple: **opening a SYN file should not grant the file control over the host system.**

## Format

The current prototype uses an inspectable JSON-based representation for development.

A minimal SYN document contains:

- `syn`: format version
- `type`: document type
- `meta`: document metadata
- `scenes`: the creative structure

Scenes can contain objects and declarative interactions.

The 0.1 format is experimental and intentionally not frozen. The implementation is expected to evolve as the medium gains timelines, animation, richer media, components, permissions, and other capabilities.

See [spec/FORMAT.md](./spec/FORMAT.md) for the current format principles.

## Development

Requirements:

- Node.js 22+
- A modern browser

Install and test:

```bash
npm test
```

The repository includes automated tests for the document model, runtime validation, and JavaScript syntax.

GitHub Actions runs the test suite on pushes to `main` and pull requests.

## Project status

**Experimental 0.3 development**

SYN is an active exploration of what a modern, portable creative medium could look like.

The current implementation is deliberately small. The foundation is being built first so future features such as timelines, animation, audio/video, reusable components, richer interactions, and AI-assisted authoring can be added without turning the format into an uncontrolled executable container.

## License

No public license has been declared yet. Until a license is added to the repository, all rights are reserved by the copyright holder.
