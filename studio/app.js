import { createSynDocument, addScene, addObject, addInteraction, serializeSynDocument } from "../src/document.js";
import { createRuntimeState, currentScene, dispatchEvent, renderScene } from "../src/runtime.js";

const state = {
  document: createSynDocument({ title: "Untitled SYN" }),
  sceneIndex: 0,
  selectedObjectId: null,
  drag: null
};
addScene(state.document, { id: "scene-1", name: "Scene 1" });

const stage = document.querySelector("#stage");
const emptyState = document.querySelector("#emptyState");
const inspector = document.querySelector("#inspectorContent");
const interactionList = document.querySelector("#interactionList");
const sceneLabel = document.querySelector("#sceneLabel");
const deleteButton = document.querySelector("#deleteObject");

function scene() { return state.document.scenes[state.sceneIndex]; }
function selected() { return scene().objects.find(item => item.id === state.selectedObjectId) ?? null; }

function addCanvasObject(kind) {
  const labels = { text: "Text", media: "Media", shape: "Shape", button: "Button" };
  const count = scene().objects.length;
  const object = addObject(scene(), {
    kind,
    label: labels[kind] ?? "Object",
    x: 60 + (count % 5) * 34,
    y: 60 + (count % 5) * 34,
    width: kind === "text" ? 220 : 160,
    height: kind === "text" ? 64 : 56,
    props: kind === "text" ? { text: "Double-click to edit" } : {}
  });
  state.selectedObjectId = object.id;
  render();
}

function selectObject(id) {
  state.selectedObjectId = id;
  document.querySelectorAll(".syn-object").forEach(el => el.classList.toggle("selected", el.dataset.id === id));
  deleteButton.disabled = !id;
  const object = selected();
  if (!object) {
    inspector.innerHTML = '<div class="inspector-empty"><span>◇</span><p>Select an object to edit its properties.</p></div>';
    return;
  }

  inspector.innerHTML = `
    <div class="eyebrow">OBJECT</div>
    <label class="field">Label<input id="objectLabel" value="${escapeHtml(object.label)}"></label>
    <div class="field-row">
      <label class="field">X<input id="objectX" type="number" value="${object.x}"></label>
      <label class="field">Y<input id="objectY" type="number" value="${object.y}"></label>
    </div>
    <div class="field-row">
      <label class="field">Width<input id="objectW" type="number" min="20" value="${object.width}"></label>
      <label class="field">Height<input id="objectH" type="number" min="20" value="${object.height}"></label>
    </div>
    <label class="field">Text<input id="objectText" value="${escapeHtml(object.props?.text ?? "")}"></label>
    <div class="interaction"><div>Kind</div><code>${escapeHtml(object.kind)}</code></div>
  `;
  for (const id of ["objectLabel","objectX","objectY","objectW","objectH","objectText"]) {
    inspector.querySelector("#" + id).addEventListener("input", updateSelectedObject);
  }
}

function updateSelectedObject() {
  const object = selected();
  if (!object) return;
  object.label = inspector.querySelector("#objectLabel").value;
  object.x = Math.max(0, Number(inspector.querySelector("#objectX").value) || 0);
  object.y = Math.max(0, Number(inspector.querySelector("#objectY").value) || 0);
  object.width = Math.max(20, Number(inspector.querySelector("#objectW").value) || 20);
  object.height = Math.max(20, Number(inspector.querySelector("#objectH").value) || 20);
  object.props = { ...object.props, text: inspector.querySelector("#objectText").value };
  render(false);
  selectObject(object.id);
}

function deleteSelectedObject() {
  if (!state.selectedObjectId) return;
  const id = state.selectedObjectId;
  scene().objects = scene().objects.filter(object => object.id !== id);
  scene().interactions = scene().interactions.filter(item => item.event.target !== id && !item.actions.some(action => action.target === id));
  state.selectedObjectId = null;
  render();
}

function addNewScene() {
  const next = state.document.scenes.length + 1;
  addScene(state.document, { name: `Scene ${next}` });
  state.sceneIndex = state.document.scenes.length - 1;
  state.selectedObjectId = null;
  render();
}

function changeScene(delta) {
  state.sceneIndex = Math.max(0, Math.min(state.document.scenes.length - 1, state.sceneIndex + delta));
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

function removeInteraction(id) {
  scene().interactions = scene().interactions.filter(item => item.id !== id);
  renderInteractions();
}

function renderInteractions() {
  if (!scene().interactions.length) {
    interactionList.innerHTML = '<div class="interaction empty">No interactions yet. Add a Button and give it something to do.</div>';
    return;
  }
  interactionList.innerHTML = scene().interactions.map(item => {
    const source = scene().objects.find(object => object.id === item.event.target);
    const action = item.actions[0];
    const targetScene = state.document.scenes.find(s => s.id === action?.target);
    const actionLabel = action?.type === "scene.goto" ? `Go to ${targetScene?.name ?? "scene"}` : action?.type ?? "none";
    return `<div class="interaction">
      <div><strong>WHEN</strong> <code>click</code> <span class="target-chip">${escapeHtml(source?.label ?? item.event.target)}</span></div>
      <div class="flow">→</div>
      <div><strong>THEN</strong> <code>${escapeHtml(actionLabel)}</code></div>
      <button class="remove-interaction" data-id="${item.id}" aria-label="Remove interaction">×</button>
    </div>`;
  }).join("");
  interactionList.querySelectorAll(".remove-interaction").forEach(button => {
    button.addEventListener("click", () => removeInteraction(button.dataset.id));
  });
}

function render(keepSelection = true) {
  stage.querySelectorAll(".syn-object").forEach(el => el.remove());
  const activeScene = scene();
  emptyState.hidden = activeScene.objects.length > 0;
  sceneLabel.textContent = activeScene.name;
  document.querySelector("#prevScene").disabled = state.sceneIndex === 0;
  document.querySelector("#nextScene").disabled = state.sceneIndex === state.document.scenes.length - 1;
  for (const object of activeScene.objects) {
    const el = document.createElement("button");
    el.className = "syn-object";
    el.dataset.id = object.id;
    el.dataset.kind = object.kind;
    el.textContent = object.props?.text || object.label;
    el.style.left = object.x + "px";
    el.style.top = object.y + "px";
    el.style.width = object.width + "px";
    el.style.height = object.height + "px";
    el.addEventListener("click", event => {
      event.stopPropagation();
      selectObject(object.id);
    });
    el.addEventListener("pointerdown", event => beginDrag(event, object));
    stage.appendChild(el);
  }
  renderInteractions();
  if (keepSelection && state.selectedObjectId && selected()) selectObject(state.selectedObjectId);
  else if (!selected()) selectObject(null);
}

function beginDrag(event, object) {
  if (event.button !== 0) return;
  event.preventDefault();
  const rect = stage.getBoundingClientRect();
  state.drag = { object, startX: event.clientX, startY: event.clientY, originX: object.x, originY: object.y, rect };
  selectObject(object.id);
  const move = e => {
    if (!state.drag) return;
    const d = state.drag;
    d.object.x = Math.max(0, Math.round(d.originX + e.clientX - d.startX));
    d.object.y = Math.max(0, Math.round(d.originY + e.clientY - d.startY));
    render(false);
  };
  const end = () => {
    state.drag = null;
    window.removeEventListener("pointermove", move);
    window.removeEventListener("pointerup", end);
    selectObject(object.id);
  };
  window.addEventListener("pointermove", move);
  window.addEventListener("pointerup", end, { once: true });
}

function preview() {
  const runtime = createRuntimeState(structuredClone(state.document));
  const win = window.open("", "_blank");
  if (!win) return alert("Allow pop-ups to preview SYN.");
  const html = `<!doctype html><html><head><title>SYN Preview</title><style>${previewCss()}</style></head><body>
    <header><strong id="title"></strong><span id="scene"></span></header><main id="stage"></main>
  </body></html>`;
  win.document.open();
  win.document.write(html);
  win.document.close();
  const mount = () => {
    if (!win.document.querySelector("#stage")) return;
    const root = win.document.querySelector("#stage");
    win.document.querySelector("#title").textContent = runtime.document.meta.title;
    win.document.querySelector("#scene").textContent = currentScene(runtime)?.name ?? "";
    renderScene(root, runtime, { onEvent: event => { dispatchEvent(runtime, event); mount(); } });
  };
  setTimeout(mount, 0);
}

function previewCss() {
  return `body{margin:0;background:#090a0d;color:#fff;font-family:system-ui}header{height:56px;padding:0 18px;display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid #242833}main{position:relative;margin:20px;min-height:600px;border:1px solid #2b2f39;border-radius:12px;background:#0c0e13;background-image:linear-gradient(#151821 1px,transparent 1px),linear-gradient(90deg,#151821 1px,transparent 1px);background-size:24px 24px}.syn-runtime-object{position:absolute;border:1px solid #3a4050;border-radius:8px;background:#171a22;color:#fff;padding:10px 14px;white-space:pre-wrap;overflow:hidden}.syn-runtime-object[data-kind=button]{background:#ff4fd8;color:#160b14;border-color:transparent;font-weight:800;cursor:pointer;text-align:center}`;
}

function exportSyn() {
  const source = serializeSynDocument(state.document);
  const blob = new Blob([source], { type: "application/vnd.syn+json" });
  const url = URL.createObjectURL(blob);
  const anchor = Object.assign(window.document.createElement("a"), {
    href: url,
    download: (state.document.meta.title || "untitled").replace(/[^a-z0-9_-]+/gi, "-").toLowerCase() + ".syn"
  });
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
    if (tool === "ai") alert("SYN AI will edit this document model directly. Runtime and authoring foundations come first.");
  });
});
stage.addEventListener("click", () => selectObject(null));
deleteButton.addEventListener("click", deleteSelectedObject);
document.querySelector("#addInteraction").addEventListener("click", addNewInteraction);
document.querySelector("#export").addEventListener("click", exportSyn);
document.querySelector("#newScene").addEventListener("click", addNewScene);
document.querySelector("#preview").addEventListener("click", preview);
document.querySelector("#prevScene").addEventListener("click", () => changeScene(-1));
document.querySelector("#nextScene").addEventListener("click", () => changeScene(1));
render();
