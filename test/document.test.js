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
    viewport: { width: 1120, height: 640 },
    assets: [],
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


test("round-trips rich object styles and navigation metadata", () => {
  const document = createSynDocument({ title: "Styled SYN", id: "doc-style" });
  const scene = addScene(document, { id: "scene-style", name: "Styled" });
  const object = addObject(scene, {
    id: "headline",
    kind: "text",
    label: "Headline",
    props: { text: "Hello" },
    styles: { fontFamily: "Georgia", fontSize: 48, color: "#ffffff", borderRadius: 12 }
  });
  object.props.target = "./other.syn";
  const parsed = parseSynDocument(serializeSynDocument(document));
  assert.deepEqual(parsed.scenes[0].objects[0].styles, object.styles);
  assert.equal(parsed.scenes[0].objects[0].props.target, "./other.syn");
});

test("creates a stable responsive viewport and asset registry", () => {
  const document = createSynDocument({ id: "doc-responsive" });
  assert.deepEqual(document.viewport, { width: 1120, height: 640 });
  assert.deepEqual(document.assets, []);
});

test("supports bounded document history with undo and redo", async () => {
  const { createHistory } = await import("../src/history.js");
  const history = createHistory({ value: { version: 1 }, limit: 3 });
  history.push({ value: { version: 2 } });
  history.push({ value: { version: 3 } });
  assert.deepEqual(history.undo(), { value: { version: 2 } });
  assert.deepEqual(history.redo(), { value: { version: 3 } });
  assert.deepEqual(history.undo(), { value: { version: 2 } });
  assert.equal(history.canUndo(), true);
});


test("accepts older 0.1 documents by applying current defaults", () => {
  const legacy = { syn: "0.1", type: "document", meta: { id: "legacy", title: "Legacy" }, scenes: [{ id: "scene", name: "Scene", objects: [], interactions: [] }] };
  const parsed = parseSynDocument(JSON.stringify(legacy));
  assert.deepEqual(parsed.viewport, { width: 1120, height: 640 });
  assert.deepEqual(parsed.assets, []);
});

test("rejects a zero-size viewport", () => {
  const value = createSynDocument({ id: "bad-viewport" });
  value.viewport.width = 0;
  assert.throws(() => serializeSynDocument(value), /Invalid SYN document/);
});
