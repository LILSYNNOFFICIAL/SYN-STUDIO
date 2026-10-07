import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = file => readFileSync(path.join(root, file), "utf8");

test("SYN Studio keeps every application menu reachable on mobile", () => {
  const app = read("studio/app.js");
  const css = read("studio/styles.css");
  assert.match(app, /const mobileMore=document\.createElement\("div"\)/);
  assert.match(app, /mobilePanel\.innerHTML=Object\.entries\(MENU_DATA\)/);
  assert.match(app, /mobileMore\.querySelector\("\.menu-trigger"\)\.addEventListener/);
  assert.match(css, /\.menu-bar > \.menu:nth-child\(n\+5\):not\(\.mobile-more-menu\)\{display:none\}/);
  assert.match(css, /\.mobile-more-menu\{display:block/);
});

test("SYN Studio mobile inspector has an accessible state and dismiss path", () => {
  const html = read("studio/index.html");
  const app = read("studio/app.js");
  const css = read("studio/styles.css");
  assert.match(html, /id="inspectorToggle"[^>]*aria-expanded="false"/);
  assert.match(app, /inspectorToggle\.setAttribute\("aria-expanded",String\(document\.body\.classList\.contains\("show-inspector"\)\)\)/);
  assert.match(app, /event\.target\.closest\("\.inspector,\.mobile-inspector-toggle"\)/);
  assert.match(css, /body\.show-inspector::before/);
});

test("SYN Studio responsive stylesheet contains phone-specific authoring controls", () => {
  const css = read("studio/styles.css");
  assert.doesNotMatch(css, /\\n/);
  assert.match(css, /@media \(max-width:560px\)/);
  assert.match(css, /\.editor-tools\{[\s\S]*overflow-x:auto/);
  assert.match(css, /\.resize-handle\s*\{\s*width:13px;\s*height:13px;\s*\}/);
  assert.match(css, /\.code-workspace\{inset:92px 0 0 !important\}/);
});


test("SYN Studio has product-level authoring controls and contextual inspector structure", () => {
  const html = read("studio/index.html");
  const app = read("studio/app.js");
  const css = read("studio/styles.css");
  assert.match(html, /id="toolbarZoomIn"/);
  assert.match(html, /id="toolbarZoomOut"/);
  assert.match(html, /id="toolbarCommand"/);
  assert.match(html, /canvas-hud-top/);
  assert.match(html, /studio-statusbar/);
  assert.match(app, /function setZoom\(value\).*zoom-value/);
  assert.match(app, /const shortcuts=\{v:"select",m:"move",h:"pan",t:"text",r:"shape"\}/);
  assert.match(app, /event\.key==="Enter".*selected\(\)\?\.kind==="text"/s);
  assert.match(app, /inspector-section/);
  assert.doesNotMatch(css, /#ff4fd8/i);
  assert.match(css, /\.inspector-heading/);
});


test("SYN Studio exposes real link actions and richer visual controls", () => {
  const app = read("studio/app.js");
  assert.match(app, /value="link\.openUrl"/);
  assert.match(app, /value="link\.openSyn"/);
  assert.match(app, /class="interaction-link"/);
  assert.match(app, /objectFontWeight/);
  assert.match(app, /objectTextAlign/);
  assert.match(app, /objectOpacity/);
  assert.match(app, /objectLetterSpacing/);
});


test("SYN Studio has actionable empty-canvas onboarding", () => {
  const html = read("studio/index.html");
  const app = read("studio/app.js");
  const css = read("studio/styles.css");
  for (const id of ["emptyText","emptyShape","emptyMedia","emptyCommands"]) assert.match(html, new RegExp('id="' + id + '"'));
  assert.match(html, /tabindex="0" aria-label="SYN canvas"/);
  assert.match(app, /emptyText.*addCanvasObject\("text"\)/s);
  assert.match(app, /emptyShape.*addCanvasObject\("shape"\)/s);
  assert.match(app, /emptyMedia.*mediaInput/s);
  assert.match(app, /emptyCommands.*openCodePalette/s);
  assert.match(css, /\.empty-state\{/);
});


test("SYN history supports a real undo and redo cycle", async () => {
  const { createHistory } = await import("../src/history.js");
  const history = createHistory({ value: 0 });
  history.push({ value: 0 });
  assert.deepEqual(history.undo({ value: 1 }), { value: 0 });
  assert.deepEqual(history.redo(), { value: 1 });
});

test("SYN runtime accepts embedded audio/video and renders media elements", async () => {
  const { createRuntimeState, renderScene } = await import("../src/runtime.js");
  const doc = {
    syn: "0.1",
    type: "document",
    meta: { id: "test-doc", title: "Media Test" },
    viewport: { width: 320, height: 240 },
    assets: [],
    scenes: [{
      id: "scene-1",
      name: "Media",
      background: "#000",
      objects: [
        { id: "audio-1", kind: "audio", label: "Audio", x: 10, y: 10, width: 120, height: 40, rotation: 0, locked: false, hidden: false, props: { src: "data:audio/mpeg;base64,AAAA", controls: true }, styles: {} },
        { id: "video-1", kind: "video", label: "Video", x: 10, y: 60, width: 160, height: 90, rotation: 0, locked: false, hidden: false, props: { src: "data:video/mp4;base64,AAAA", controls: true }, styles: {} }
      ],
      interactions: []
    }]
  };
  const state = createRuntimeState(doc);
  const createdTags = [];
  const container = {
    clientWidth: 320,
    clientHeight: 240,
    style: {},
    ownerDocument: {
      createDocumentFragment() { return { children: [], appendChild(child) { this.children.push(child); } }; },
      createElement(tag) {
        return {
          tagName: tag.toUpperCase(),
          _created: (createdTags.push(tag), true),
          style: {},
          dataset: {},
          setAttribute() {},
          appendChild() {},
          addEventListener() {}
        };
      }
    },
    replaceChildren() {},
    appendChild() {}
  };
  renderScene(container, state);
  assert.equal(container.style.background, "#000");
  assert.ok(createdTags.includes("audio"));
  assert.ok(createdTags.includes("video"));
});

test("SYN runtime preserves object-level hidden state until an explicit show action", async () => {
  const { createRuntimeState, renderScene } = await import("../src/runtime.js");
  const doc = {
    syn: "0.1", type: "document", meta: { id: "hidden-doc", title: "Hidden Test" },
    viewport: { width: 200, height: 120 }, assets: [],
    scenes: [{
      id: "scene-1", name: "Scene", background: "#000",
      objects: [{ id: "hidden-1", kind: "shape", label: "Hidden", x: 0, y: 0, width: 50, height: 50, rotation: 0, locked: false, hidden: true, props: {}, styles: {} }],
      interactions: []
    }]
  };
  const state = createRuntimeState(doc);
  let hidden = false;
  const container = {
    clientWidth: 200, clientHeight: 120, style: {}, ownerDocument: {
      createDocumentFragment() { return { appendChild() {} }; },
      createElement() { return { style: {}, dataset: {}, setAttribute() {}, appendChild() {}, addEventListener() {}, set hidden(value) { hidden = value; } }; }
    },
    replaceChildren() {}, appendChild() {}
  };
  renderScene(container, state);
  assert.equal(hidden, true);
});


test("SYN scene duplication preserves and remaps internal interactions", async () => {
  const { createSynDocument, addScene, addObject, addInteraction } = await import("../src/document.js");
  const doc = createSynDocument({ title: "Duplicate Test" });
  const scene = addScene(doc, { id: "scene-1", name: "Scene 1" });
  const button = addObject(scene, { id: "button-1", kind: "button", label: "Go" });
  const target = addObject(scene, { id: "target-1", kind: "text", label: "Target" });
  addInteraction(scene, { event: { type: "click", target: button.id }, actions: [{ type: "object.setText", target: target.id, value: "Changed" }] });
  assert.equal(scene.interactions.length, 1);
  assert.equal(button.id, "button-1");
  assert.equal(target.id, "target-1");
});

test("SYN Studio app code remaps duplicated scene object and interaction targets", () => {
  const app = read("studio/app.js");
  assert.match(app, /const idMap=new Map\(current\.objects\.map/);
  assert.match(app, /event:\{\.\.\.item\.event,target:idMap\.get/);
  assert.match(app, /actions:item\.actions\.map/);
  assert.match(app, /action\.type === "scene\.goto" && action\.target === current\.id/);
  assert.match(app, /state\.history=createHistory\(previousDocument/);
});

test("SYN Studio clears stale multi-selection when changing scenes", () => {
  const app = read("studio/app.js");
  assert.match(app, /function changeScene\(delta\)[\s\S]*state\.selectedObjectIds = \[\][\s\S]*render\(\)/);
});

test("SYN runtime rejects executable link schemes", async () => {
  const { createRuntimeState } = await import("../src/runtime.js");
  const doc = {
    syn: "0.1", type: "document", meta: { id: "security-doc", title: "Security Test" },
    viewport: { width: 100, height: 100 }, assets: [],
    scenes: [{
      id: "scene-1", name: "Scene", objects: [{ id: "button", kind: "button", label: "Button", x: 0, y: 0, width: 50, height: 30, rotation: 0, locked: false, hidden: false, props: {}, styles: {} }],
      interactions: [{ id: "i", event: { type: "click", target: "button" }, actions: [{ type: "link.openSyn", target: "javascript:alert(1)" }] }]
    }]
  };
  assert.throws(() => createRuntimeState(doc), /relative target/);
});

test("SYN runtime carries editor rotation and typography styles into preview", async () => {
  const { createRuntimeState, renderScene } = await import("../src/runtime.js");
  const doc = {
    syn: "0.1", type: "document", meta: { id: "style-doc", title: "Style Test" },
    viewport: { width: 200, height: 120 }, assets: [],
    scenes: [{
      id: "scene-1", name: "Scene", background: "#000",
      objects: [{ id: "shape", kind: "shape", label: "Styled", x: 10, y: 10, width: 60, height: 40, rotation: 27, locked: false, hidden: false, props: {}, styles: { padding: 8, textTransform: "uppercase" } }],
      interactions: []
    }]
  };
  const state = createRuntimeState(doc);
  let created;
  const container = {
    clientWidth: 200, clientHeight: 120, style: {}, ownerDocument: {
      createDocumentFragment() { return { appendChild() {} }; },
      createElement() { created = { style: {}, dataset: {}, setAttribute() {}, appendChild() {}, addEventListener() {} }; return created; }
    },
    replaceChildren() {}, appendChild() {}
  };
  renderScene(container, state);
  assert.equal(created.style.transform, "rotate(27deg)");
  assert.equal(created.style.padding, "8px");
  assert.equal(created.style.textTransform, "uppercase");
});
