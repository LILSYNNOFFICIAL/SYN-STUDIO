const state = {
  title: "Untitled SYN",
  objects: [],
  interactions: []
};

const stage = document.querySelector("#stage");
const emptyState = document.querySelector("#emptyState");
const inspector = document.querySelector("#inspectorContent");
const interactionList = document.querySelector("#interactionList");

function addObject(kind) {
  const labels = {
    text: "Text",
    media: "Media",
    shape: "Shape",
    button: "Button"
  };
  const object = {
    id: crypto.randomUUID(),
    kind,
    label: labels[kind] ?? "Object",
    x: 80 + state.objects.length * 18,
    y: 80 + state.objects.length * 18
  };

  state.objects.push(object);
  render();
  selectObject(object.id);
}

function selectObject(id) {
  document.querySelectorAll(".syn-object").forEach(el => {
    el.classList.toggle("selected", el.dataset.id === id);
  });

  const object = state.objects.find(item => item.id === id);
  if (!object) return;

  inspector.innerHTML = `
    <div class="eyebrow">OBJECT</div>
    <strong>${escapeHtml(object.label)}</strong>
    <div style="height:12px"></div>
    <div class="interaction">
      <div>Type</div>
      <code>${escapeHtml(object.kind)}</code>
    </div>
    <div style="height:8px"></div>
    <div class="interaction">
      <div>Position</div>
      <code>${object.x}, ${object.y}</code>
    </div>
  `;
}

function addInteraction() {
  const button = state.objects.find(item => item.kind === "button");
  if (!button) {
    alert("Add a Button first.");
    return;
  }

  state.interactions.push({
    id: crypto.randomUUID(),
    event: `${button.label} clicked`,
    action: "go to next scene"
  });
  renderInteractions();
}

function renderInteractions() {
  if (!state.interactions.length) {
    interactionList.innerHTML = '<div class="interaction empty">No interactions yet. Add a button and give it something to do.</div>';
    return;
  }

  interactionList.innerHTML = state.interactions.map(item => `
    <div class="interaction">
      <strong>WHEN</strong> <code>${escapeHtml(item.event)}</code>
      <span> → </span>
      <strong>THEN</strong> <code>${escapeHtml(item.action)}</code>
    </div>
  `).join("");
}

function render() {
  stage.querySelectorAll(".syn-object").forEach(el => el.remove());
  emptyState.hidden = state.objects.length > 0;

  for (const object of state.objects) {
    const el = document.createElement("button");
    el.className = "syn-object";
    el.dataset.id = object.id;
    el.dataset.kind = object.kind;
    el.textContent = object.label;
    el.style.left = object.x + "px";
    el.style.top = object.y + "px";
    el.addEventListener("click", () => selectObject(object.id));
    stage.appendChild(el);
  }

  renderInteractions();
}

function exportSyn() {
  const synDocument = {
    syn: "0.1",
    type: "document",
    meta: { id: crypto.randomUUID(), title: state.title },
    scenes: [{
      id: "scene-1",
      name: "Scene 1",
      objects: state.objects,
      interactions: state.interactions
    }]
  };

  const blob = new Blob([JSON.stringify(synDocument, null, 2) + "\n"], {
    type: "application/vnd.syn+json"
  });
  const url = URL.createObjectURL(blob);
  const anchor = Object.assign(window.document.createElement("a"), {
    href: url,
    download: "untitled.syn"
  });
  anchor.click();
  URL.revokeObjectURL(url);
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, char => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;"
  }[char]));
}

document.querySelectorAll(".tool").forEach(button => {
  button.addEventListener("click", () => {
    document.querySelectorAll(".tool").forEach(item => item.classList.remove("active"));
    button.classList.add("active");

    const tool = button.dataset.tool;
    if (["text", "media", "shape", "button"].includes(tool)) addObject(tool);
    if (tool === "ai") alert("SYN AI is planned as a native authoring layer, not a separate export step.");
  });
});

document.querySelector("#addInteraction").addEventListener("click", addInteraction);
document.querySelector("#export").addEventListener("click", exportSyn);
document.querySelector("#newScene").addEventListener("click", () => alert("Scene graph is next. This prototype keeps one scene."));
document.querySelector("#preview").addEventListener("click", () => alert("Preview mode is next. The same SYN scene graph will drive it."));

render();
