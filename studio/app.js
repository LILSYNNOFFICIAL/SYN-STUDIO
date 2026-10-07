import { createSynDocument, addScene, addObject, addInteraction, serializeSynDocument } from "../src/document.js";
import { createRuntimeState, currentScene, dispatchEvent, renderScene } from "../src/runtime.js";

const state = {
  document: createSynDocument({ title: "Untitled SYN" }),
  sceneIndex: 0,
  selectedObjectId: null
};
addScene(state.document, { id: "scene-1", name: "Scene 1" });

const stage = document.querySelector("#stage");
const emptyState = document.querySelector("#emptyState");
const inspector = document.querySelector("#inspectorContent");
const interactionList = document.querySelector("#interactionList");
const sceneLabel = document.querySelector("#sceneLabel");

function scene() { return state.document.scenes[state.sceneIndex]; }

function addCanvasObject(kind) {
  const labels = { text: "Text", media: "Media", shape: "Shape", button: "Button" };
  const object = addObject(scene(), {
    kind,
    label: labels[kind] ?? "Object",
    x: 80 + scene().objects.length * 18,
    y: 80 + scene().objects.length * 18,
    props: kind === "text" ? { text: "Double-click to edit" } : {}
  });
  state.selectedObjectId = object.id;
  render();
}

function selectObject(id) {
  state.selectedObjectId = id;
  document.querySelectorAll(".syn-object").forEach(el => el.classList.toggle("selected", el.dataset.id === id));
  const object = scene().objects.find(item => item.id === id);
  if (!object) return;

  inspector.innerHTML = `
    <div class="eyebrow">OBJECT</div>
    <label class="field">Label<input id="objectLabel" value="${escapeHtml(object.label)}"></label>
    <label class="field">X<input id="objectX" type="number" value="${object.x}"></label>
    <label class="field">Y<input id="objectY" type="number" value="${object.y}"></label>
    <div class="interaction"><div>Type</div><code>${escapeHtml(object.kind)}</code></div>
  `;
  for (const id of ["objectLabel","objectX","objectY"]) {
    inspector.querySelector("#" + id).addEventListener("input", updateSelectedObject);
  }
}

function updateSelectedObject() {
  const object = scene().objects.find(item => item.id === state.selectedObjectId);
  if (!object) return;
  object.label = inspector.querySelector("#objectLabel").value;
  object.x = Number(inspector.querySelector("#objectX").value) || 0;
  object.y = Number(inspector.querySelector("#objectY").value) || 0;
  render();
  selectObject(object.id);
}

function addNewScene() {
  const next = state.document.scenes.length + 1;
  addScene(state.document, { id: crypto.randomUUID(), name: `Scene ${next}` });
  state.sceneIndex = state.document.scenes.length - 1;
  state.selectedObjectId = null;
  render();
}

function addNewInteraction() {
  const button = scene().objects.find(item => item.kind === "button");
  if (!button) return alert("Add a Button first.");

  const target = state.document.scenes[state.sceneIndex + 1];
  addInteraction(scene(), {
    event: { type: "click", target: button.id },
    actions: [{ type: target ? "scene.goto" : "scene.next", ...(target ? { target: target.id } : {}) }]
  });
  renderInteractions();
}

function renderInteractions() {
  if (!scene().interactions.length) {
    interactionList.innerHTML = '<div class="interaction empty">No interactions yet. Add a Button and give it something to do.</div>';
    return;
  }
  interactionList.innerHTML = scene().interactions.map(item => {
    const action = item.actions[0];
    return `<div class="interaction"><strong>WHEN</strong> <code>${escapeHtml(item.event.type)} → ${escapeHtml(item.event.target)}</code><span> → </span><strong>THEN</strong> <code>${escapeHtml(action?.type ?? "none")}</code></div>`;
  }).join("");
}

function render() {
  stage.querySelectorAll(".syn-object").forEach(el => el.remove());
  emptyState.hidden = scene().objects.length > 0;
  sceneLabel.textContent = scene().name;
  for (const object of scene().objects) {
    const el = document.createElement("button");
    el.className = "syn-object";
    el.dataset.id = object.id;
    el.dataset.kind = object.kind;
    el.textContent = object.props?.text ?? object.label;
    el.style.left = object.x + "px";
    el.style.top = object.y + "px";
    el.addEventListener("click", event => { event.stopPropagation(); selectObject(object.id); });
    stage.appendChild(el);
  }
  renderInteractions();
  if (state.selectedObjectId) selectObject(state.selectedObjectId);
}

function preview() {
  const runtime = createRuntimeState(structuredClone(state.document));
  const win = window.open("", "_blank");
  if (!win) return alert("Allow pop-ups to preview SYN.");
  win.document.write(`<!doctype html><html><head><title>SYN Preview</title><style>${previewCss()}</style></head><body><header><strong id="title"></strong><span id="scene"></span></header><main id="stage"></main><script type="module">window.name = "syn-preview";<\/script></body></html>`);
  win.document.close();
  win.addEventListener("load", () => mountPreview(win, runtime), { once: true });
  setTimeout(() => mountPreview(win, runtime), 50);
}

function mountPreview(win, runtime) {
  if (!win.document.querySelector("#stage")) return;
  const mount = () => {
    const root = win.document.querySelector("#stage");
    win.document.querySelector("#title").textContent = runtime.document.meta.title;
    win.document.querySelector("#scene").textContent = currentScene(runtime)?.name ?? "";
    renderScene(root, runtime, { onEvent: event => { dispatchEvent(runtime, event); mount(); } });
  };
  mount();
}

function previewCss() {
  return `body{margin:0;background:#090a0d;color:#fff;font-family:system-ui}header{height:56px;padding:0 18px;display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid #242833}main{position:relative;margin:20px;min-height:600px;border:1px solid #2b2f39;border-radius:12px;background:#0c0e13;background-image:linear-gradient(#151821 1px,transparent 1px),linear-gradient(90deg,#151821 1px,transparent 1px);background-size:24px 24px}.syn-runtime-object{position:absolute;border:1px solid #3a4050;border-radius:8px;background:#171a22;color:#fff;padding:10px 14px;white-space:pre-wrap;overflow:hidden}.syn-runtime-object[data-kind=button]{background:#ff4fd8;color:#160b14;border-color:transparent;font-weight:800;cursor:pointer;text-align:center}`;
}

function exportSyn() {
  const source = serializeSynDocument(state.document);
  const blob = new Blob([source], { type: "application/vnd.syn+json" });
  const url = URL.createObjectURL(blob);
  const anchor = Object.assign(window.document.createElement("a"), { href: url, download: (state.document.meta.title || "untitled").replace(/[^a-z0-9_-]+/gi, "-").toLowerCase() + ".syn" });
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, char => ({ "&":"&amp;", "<":"&lt;", ">":"&gt;", '"':"&quot;", "'":"&#39;" }[char]));
}

document.querySelectorAll(".tool").forEach(button => {
  button.addEventListener("click", () => {
    document.querySelectorAll(".tool").forEach(item => item.classList.remove("active"));
    button.classList.add("active");
    const tool = button.dataset.tool;
    if (["text","media","shape","button"].includes(tool)) addCanvasObject(tool);
    if (tool === "scene") addNewScene();
    if (tool === "ai") alert("SYN AI will operate directly on this document model. The runtime foundation comes first.");
  });
});
document.querySelector("#addInteraction").addEventListener("click", addNewInteraction);
document.querySelector("#export").addEventListener("click", exportSyn);
document.querySelector("#newScene").addEventListener("click", addNewScene);
document.querySelector("#preview").addEventListener("click", preview);
render();
