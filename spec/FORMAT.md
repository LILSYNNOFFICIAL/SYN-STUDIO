# SYN Format 0.1

SYN is intended to be a shareable creative document, not a generic archive.

The first prototype uses a declarative JSON representation so the format can be inspected, diffed, generated, and tested while the model is still evolving.

## Core shape

```json
{
  "syn": "0.1",
  "type": "document",
  "meta": {
    "id": "uuid",
    "title": "Untitled SYN"
  },
  "scenes": [
    {
      "id": "scene-1",
      "name": "Scene 1",
      "objects": [],
      "interactions": []
    }
  ]
}
```

### Design principles

1. **A SYN file is data, not authority.** Opening it must not grant filesystem, shell, or unrestricted network access.
2. **Behavior is declarative first.** Interactions are expressed as event → action relationships before any general-purpose code is considered.
3. **The document is the creative object.** A birthday card, interactive story, presentation, music experience, portfolio, website, or game can share the same underlying model.
4. **AI edits the document model.** AI should create and transform SYN objects directly rather than producing an unrelated pile of exports.
5. **The format must remain inspectable.** Early versions should be human-readable and deterministic where practical.
6. **The runtime is separate from the file.** A SYN file should describe an experience. The viewer/renderer supplies the sandboxed execution environment.

This is an experimental 0.1 model, not a frozen standard.


## Runtime action model

The prototype runtime uses a deliberately constrained declarative action vocabulary:

- `scene.next`
- `scene.goto`
- `object.show`
- `object.hide`
- `object.setText`

A document cannot execute arbitrary JavaScript through this model. Runtime validation rejects unknown actions and broken targets. Media is currently intended to be embedded as image data rather than fetched from arbitrary network URLs.

This vocabulary is experimental and will evolve with the format.
