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
  if (kind === "media") {
    document.querySelector("#mediaInput").click();
    return;
  }
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

function renderDocumentInspector() {
  inspector.innerHTML = `
    <div class="eyebrow">DOCUMENT</div>
    <label class="field">Title<input id="documentTitle" value="${escapeHtml(state.document.meta.title)}"></label>
    <label class="field">Scene name<input id="sceneNameInput" value="${escapeHtml(scene().name)}"></label>
    <div class="interaction"><div>Scenes</div><code>${state.document.scenes.length}</code></div>
    <div class="interaction"><div>Objects</div><code>${scene().objects.length}</code></div>
  `;
  inspector.querySelector("#documentTitle").addEventListener("input", event => {
    state.document.meta.title = event.target.value || "Untitled SYN";
  });
  inspector.querySelector("#sceneNameInput").addEventListener("input", event => {
    scene().name = event.target.value || "Scene";
    sceneLabel.textContent = scene().name;
  });
}

function selectObject(id) {
  state.selectedObjectId = id;
  document.querySelectorAll(".syn-object").forEach(el => el.classList.toggle("selected", el.dataset.id === id));
  deleteButton.disabled = !id;
  const object = selected();
  if (!object) { renderDocumentInspector(); return; }

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
  const objectOptions = scene().objects.map(object => `<option value="${escapeHtml(object.id)}">${escapeHtml(object.label)}</option>`).join("");
  const sceneOptions = state.document.scenes.map(target => `<option value="${escapeHtml(target.id)}">${escapeHtml(target.name)}</option>`).join("");
  interactionList.innerHTML = scene().interactions.map(item => {
    const action = item.actions[0] ?? { type: "scene.next" };
    const source = scene().objects.find(object => object.id === item.event.target);
    const targetOptions = action.type === "scene.goto" ? sceneOptions : objectOptions;
    return `<div class="interaction">
      <div class="interaction-rule"><strong>WHEN</strong> <span class="target-chip">${escapeHtml(source?.label ?? item.event.target)}</span> <code>clicked</code></div>
      <span class="flow">→</span>
      <div class="interaction-rule">
        <strong>THEN</strong>
        <select class="interaction-action" data-id="${item.id}">
          <option value="scene.next" ${action.type === "scene.next" ? "selected" : ""}>Next scene</option>
          <option value="scene.goto" ${action.type === "scene.goto" ? "selected" : ""}>Go to scene</option>
          <option value="object.show" ${action.type === "object.show" ? "selected" : ""}>Show object</option>
          <option value="object.hide" ${action.type === "object.hide" ? "selected" : ""}>Hide object</option>
          <option value="object.setText" ${action.type === "object.setText" ? "selected" : ""}>Set object text</option>
        </select>
        ${action.type !== "scene.next" ? `<select class="interaction-target" data-id="${item.id}">${targetOptions}</select>` : ""}
        ${action.type === "object.setText" ? `<input class="interaction-value" data-id="${item.id}" value="${escapeHtml(action.value ?? "")}" placeholder="Text">` : ""}
      </div>
      <button class="remove-interaction" data-id="${item.id}" aria-label="Remove interaction">×</button>
    </div>`;
  }).join("");

  scene().interactions.forEach(item => {
    const actionSelect = interactionList.querySelector(`.interaction-action[data-id="${item.id}"]`);
    if (!actionSelect) return;
    actionSelect.addEventListener("change", () => {
      const action = item.actions[0] ?? { type: "scene.next" };
      action.type = actionSelect.value;
      delete action.target;
      delete action.value;
      if (action.type === "scene.goto") action.target = state.document.scenes[state.sceneIndex + 1]?.id ?? state.document.scenes[0]?.id;
      if (["object.show","object.hide","object.setText"].includes(action.type)) action.target = scene().objects[0]?.id;
      if (action.type === "object.setText") action.value = "";
      item.actions = [action];
      renderInteractions();
    });
  });

  interactionList.querySelectorAll(".interaction-target").forEach(select => {
    select.addEventListener("change", () => {
      const item = scene().interactions.find(value => value.id === select.dataset.id);
      if (item) item.actions[0].target = select.value;
    });
    const action = scene().interactions.find(value => value.id === select.dataset.id)?.actions[0];
    if (action?.target) select.value = action.target;
  });

  interactionList.querySelectorAll(".interaction-value").forEach(input => {
    input.addEventListener("input", () => {
      const item = scene().interactions.find(value => value.id === input.dataset.id);
      if (item) item.actions[0].value = input.value;
    });
  });

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

document.querySelector("#mediaInput").addEventListener("change", async event => {
  const file = event.target.files?.[0];
  if (!file) return;
  if (!file.type.startsWith("image/")) {
    alert("SYN currently embeds images as media assets.");
    event.target.value = "";
    return;
  }
  const reader = new FileReader();
  reader.onload = () => {
    const count = scene().objects.length;
    const object = addObject(scene(), {
      kind: "media",
      label: file.name.replace(/\.[^.]+$/, "") || "Image",
      x: 60 + (count % 4) * 40,
      y: 60 + (count % 4) * 40,
      width: 220,
      height: 160,
      props: { src: String(reader.result) }
    });
    state.selectedObjectId = object.id;
    render();
  };
  reader.readAsDataURL(file);
  event.target.value = "";
});

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
document.querySelector("#newDocument").addEventListener("click", () => {
  if (!confirm("Start a new SYN document? Unsaved work will be lost.")) return;
  state.document = createSynDocument({ title: "Untitled SYN" });
  addScene(state.document, { name: "Scene 1" });
  state.sceneIndex = 0;
  state.selectedObjectId = null;
  render();
});
document.querySelector("#openSyn").addEventListener("change", async event => {
  const file = event.target.files?.[0];
  if (!file) return;
  try {
    const source = await file.text();
    const runtime = createRuntimeState(JSON.parse(source));
    state.document = runtime.document;
    state.sceneIndex = 0;
    state.selectedObjectId = null;
    render();
  } catch (error) {
    alert(error instanceof Error ? error.message : "Unable to open SYN document.");
  }
  event.target.value = "";
});

document.querySelector("#preview").addEventListener("click", preview);
document.querySelector("#prevScene").addEventListener("click", () => changeScene(-1));
document.querySelector("#nextScene").addEventListener("click", () => changeScene(1));
render();
