# SYN Studio
SYN Studio is the visual authoring environment for the SYN 0.1 creative document format.

## Rust-first architecture
The editor UI, document model, state, history, source editor and media workflow are now implemented in Rust with Dioxus. The web target compiles to WebAssembly and keeps the browser build first-class. Dioxus also provides a path to native desktop and mobile targets from the same Rust codebase.

The Rust-WebUI project was evaluated as well. It is a legitimate MIT-licensed Rust wrapper around the WebUI C library and is excellent as a lightweight native browser/WebView shell, but it is not itself a replacement for a hosted browser frontend. SYN Studio therefore uses Dioxus for the web/editor surface and leaves WebUI as a future native-shell integration boundary.

## Product changes
- Smaller top-level command system with grouped searchable panels.
- Insert > Video opens a real file workflow and embeds selected media bytes.
- Save persists the document locally in the browser.
- Export creates a SYN file.
- Design and Code views edit the same Rust document model.
- Undo and redo are document-state based.
- Showcase is a real multi-scene SYN document, not a static marketing mock.
- Responsive chrome keeps the artboard usable on narrow screens.
