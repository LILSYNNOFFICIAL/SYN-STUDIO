function makeId() {
  if (globalThis.crypto && typeof globalThis.crypto.randomUUID === "function") return globalThis.crypto.randomUUID();
  return "syn-" + Date.now().toString(36) + "-" + Math.random().toString(36).slice(2);
}

export function createSynDocument({ title = "Untitled SYN", id = makeId() } = {}) {
  return {
    syn: "0.1",
    type: "document",
    meta: { id, title },
    viewport: { width: 1120, height: 640 },
    assets: [],
    scenes: []
  };
}

export function addScene(document, { name = "Scene", id = makeId(), background = "#0b0d12" } = {}) {
  const scene = { id, name, background, objects: [], interactions: [] };
  document.scenes.push(scene);
  return scene;
}

export function addObject(scene, {
  kind,
  label = "Object",
  id = makeId(),
  x = 80,
  y = 80,
  width = 180,
  height = 60,
  props = {},
  styles = {}
} = {}) {
  if (!kind) throw new TypeError("Object kind is required");
  const object = { id, kind, label, x, y, width, height, rotation: 0, locked: false, hidden: false, props, styles };
  scene.objects.push(object);
  return object;
}

export function addInteraction(scene, { event, actions = [], id = makeId() } = {}) {
  if (!event || typeof event !== "object" || !event.type || !event.target) {
    throw new TypeError("Interaction event requires type and target");
  }
  const interaction = { id, event, actions };
  scene.interactions.push(interaction);
  return interaction;
}

export function serializeSynDocument(document) {
  validateSynDocument(document);
  return JSON.stringify(document, null, 2) + "\n";
}

export function parseSynDocument(source) {
  let document;
  try {
    document = JSON.parse(source);
  } catch {
    throw new Error("Invalid SYN document: malformed JSON");
  }
  if (!document.viewport) document.viewport = { width: 1120, height: 640 };
  if (!Array.isArray(document.assets)) document.assets = [];
  validateSynDocument(document);
  return document;
}

function validateSynDocument(document) {
  if (!document || document.syn !== "0.1" || document.type !== "document" || !document.meta ||
      typeof document.meta.id !== "string" || typeof document.meta.title !== "string" ||
      !document.viewport || !Number.isFinite(document.viewport.width) || !Number.isFinite(document.viewport.height) ||
      document.viewport.width <= 0 || document.viewport.height <= 0 ||
      !Array.isArray(document.assets) || !Array.isArray(document.scenes)) {
    throw new Error("Invalid SYN document");
  }
  for (const scene of document.scenes) {
    if (!scene || typeof scene.id !== "string" || typeof scene.name !== "string" ||
        !Array.isArray(scene.objects) || !Array.isArray(scene.interactions)) {
      throw new Error("Invalid SYN document");
    }
    for (const object of scene.objects) {
      if (!object || typeof object.id !== "string" || typeof object.kind !== "string" ||
          !Number.isFinite(object.x) || !Number.isFinite(object.y) ||
          !Number.isFinite(object.width) || !Number.isFinite(object.height) ||
          object.width < 0 || object.height < 0) {
        throw new Error("Invalid SYN object");
      }
      if (object.rotation != null && !Number.isFinite(object.rotation)) throw new Error("Invalid SYN object rotation");
      if (object.locked != null && typeof object.locked !== "boolean") throw new Error("Invalid SYN object lock state");
      if (object.hidden != null && typeof object.hidden !== "boolean") throw new Error("Invalid SYN object visibility state");
    }
  }
}
