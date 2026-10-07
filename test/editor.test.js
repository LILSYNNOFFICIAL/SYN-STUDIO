import test from "node:test";
import assert from "node:assert/strict";
import { createSynDocument, addScene, addObject } from "../src/document.js";
import { moveObject, resizeObject, duplicateObject, reorderObject, hitTest } from "../src/editor.js";

test("moves and resizes objects inside the document viewport", () => {
  const document = createSynDocument({ id: "editor-doc" });
  const scene = addScene(document, { id: "editor-scene", name: "Editor" });
  const object = addObject(scene, { id: "object-1", kind: "shape", x: 20, y: 30, width: 100, height: 80 });
  moveObject(object, 1080, 620, document.viewport);
  assert.deepEqual({ x: object.x, y: object.y }, { x: 1020, y: 560 });
  resizeObject(object, 500, 500, document.viewport);
  assert.deepEqual({ width: object.width, height: object.height }, { width: 100, height: 80 });
});

test("duplicates and reorders objects deterministically", () => {
  const document = createSynDocument({ id: "editor-doc-2" });
  const scene = addScene(document, { id: "editor-scene-2", name: "Editor" });
  addObject(scene, { id: "a", kind: "shape", x: 0, y: 0 });
  addObject(scene, { id: "b", kind: "shape", x: 100, y: 0 });
  const copy = duplicateObject(scene, "a");
  assert.notEqual(copy.id, "a");
  assert.equal(scene.objects.length, 3);
  assert.equal(reorderObject(scene, "a", "front"), true);
  assert.equal(scene.objects.at(-1).id, "a");
  assert.equal(reorderObject(scene, "a", "back"), true);
  assert.equal(scene.objects[0].id, "a");
});

test("hit tests the topmost object", () => {
  const document = createSynDocument({ id: "editor-doc-3" });
  const scene = addScene(document, { id: "editor-scene-3", name: "Editor" });
  addObject(scene, { id: "back", kind: "shape", x: 10, y: 10, width: 100, height: 100 });
  addObject(scene, { id: "front", kind: "shape", x: 50, y: 50, width: 100, height: 100 });
  assert.equal(hitTest(scene, 75, 75).id, "front");
  assert.equal(hitTest(scene, 300, 300), null);
});
