# SYN Studio

SYN Studio is the Rust + Iced visual authoring environment for the SYN creative document format.

## Rust-first architecture

The editor UI, document model, state, history, source editor, canvas renderer, animation system, and interaction model are implemented in Rust with Iced. The web target compiles to WebAssembly, while the same architecture remains suitable for native desktop and mobile targets.

## Product direction

SYN Studio is being built as a serious creative workstation, not a decorative mockup:

- Refined Iced-native visual system with compact, layered chrome.
- Design, Motion, Architecture, Media, Code, AI, and Publish workspaces share one typed Rust document model.
- Interactive Iced Canvas rendering with object selection, 3D projection, drawing, zoom, and touch pinch support.
- Motion editing with tracks, keyframes, easing, looping, onion-skin state, and playback.
- Source editing and validation operate on the same serialized SYN document.
- Save persists the document locally in the browser.
- Export creates a portable `.syn` project.
- The Inspector is a right-side dock and remains closed until explicitly opened or an object is selected.
- The artboard automatically fits its available viewport, including narrow phone-sized WebAssembly viewports, while preserving manual zoom.
