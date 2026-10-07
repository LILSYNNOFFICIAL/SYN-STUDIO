import test from "node:test";
import assert from "node:assert/strict";
import {
  createSynDocument,
  addScene,
  addObject,
  addInteraction,
  serializeSynDocument,
  parseSynDocument
} from "../src/document.js";

test("creates a minimal SYN document with identity and empty scene graph", () => {
  const document = createSynDocument({
    title: "Hello SYN",
    id: "00000000-0000-0000-0000-000000000001"
  });
  assert.deepEqual(document, {
    syn: "0.1",
    type: "document",
    meta: { id: "00000000-0000-0000-0000-000000000001", title: "Hello SYN" },
    scenes: []
  });
});

test("builds scenes, objects, and declarative interactions", () => {
  const document = createSynDocument({ id: "doc-1" });
  const scene = addScene(document, { id: "scene-1", name: "Card" });
  const button = addObject(scene, { id: "button-1", kind: "button", label: "Open" });
  const image = addObject(scene, { id: "image-1", kind: "media", label: "Photo" });
  const interaction = addInteraction(scene, {
    id: "interaction-1",
    event: { type: "click", target: button.id },
    actions: [{ type: "object.show", target: image.id }]
  });
  assert.equal(scene, document.scenes[0]);
  assert.equal(button.kind, "button");
  assert.equal(interaction.actions[0].target, "image-1");
  assert.equal(scene.interactions.length, 1);
});

test("serializes and parses a SYN document without changing its data", () => {
  const document = createSynDocument({ title: "Round trip", id: "doc-2" });
  addScene(document, { id: "scene-1", name: "Main" });
  const serialized = serializeSynDocument(document);
  assert.equal(serialized, JSON.stringify(document, null, 2) + "\n");
  assert.deepEqual(parseSynDocument(serialized), document);
});

test("rejects malformed SYN documents at the public parsing boundary", () => {
  assert.throws(() => parseSynDocument("{}"), /Invalid SYN document/);
  assert.throws(
    () => parseSynDocument('{"syn":"0.1","type":"document","meta":{},"scenes":[]}'),
    /Invalid SYN document/
  );
});
