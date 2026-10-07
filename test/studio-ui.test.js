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
