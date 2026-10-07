import test from "node:test";
import assert from "node:assert/strict";
import {
  createRuntimeState,
  currentScene,
  applyAction,
  dispatchEvent,
  validateRuntimeDocument
} from "../src/runtime.js";

const demo = {
  syn: "0.1",
  type: "document",
  meta: { id: "doc-1", title: "Demo" },
  scenes: [
    {
      id: "scene-a",
      name: "A",
      objects: [{ id: "button-1", kind: "button", label: "Next" }],
      interactions: [{
        id: "interaction-1",
        event: { type: "click", target: "button-1" },
        actions: [{ type: "scene.goto", target: "scene-b" }]
      }]
    },
    { id: "scene-b", name: "B", objects: [], interactions: [] }
  ]
};

test("validates and starts at the first scene", () => {
  const state = createRuntimeState(demo);
  assert.equal(currentScene(state).id, "scene-a");
});

test("dispatches a declarative scene transition", () => {
  const state = createRuntimeState(demo);
  assert.equal(dispatchEvent(state, { type: "click", target: "button-1" }), true);
  assert.equal(currentScene(state).id, "scene-b");
});

test("supports bounded next and object state actions", () => {
  const state = createRuntimeState(demo);
  assert.equal(applyAction(state, { type: "scene.next" }), true);
  assert.equal(applyAction(state, { type: "scene.next" }), true);
  assert.equal(state.sceneIndex, 1);
  assert.equal(applyAction(state, { type: "object.hide", target: "x" }), true);
  assert.equal(state.visible.get("x"), false);
  assert.equal(applyAction(state, { type: "object.setText", target: "x", value: "Hello" }), true);
  assert.equal(state.text.get("x"), "Hello");
});

test("rejects unknown executable actions", () => {
  const invalid = structuredClone(demo);
  invalid.scenes[0].interactions[0].actions = [{ type: "run.js" }];
  assert.throws(() => validateRuntimeDocument(invalid), /Unsupported SYN action/);
});
