const ACTIONS = new Set([
  "scene.next",
  "scene.goto",
  "object.show",
  "object.hide",
  "object.setText"
]);

export function validateRuntimeDocument(document) {
  if (
    !document ||
    document.syn !== "0.1" ||
    document.type !== "document" ||
    !document.meta ||
    typeof document.meta.id !== "string" ||
    typeof document.meta.title !== "string" ||
    !Array.isArray(document.scenes)
  ) {
    throw new Error("Invalid SYN document");
  }
  for (const scene of document.scenes) {
    if (!scene || typeof scene.id !== "string" || typeof scene.name !== "string") {
      throw new Error("Invalid SYN scene");
    }
    if (!Array.isArray(scene.objects) || !Array.isArray(scene.interactions)) {
      throw new Error("Invalid SYN scene graph");
    }
    for (const object of scene.objects) {
      if (!object || typeof object.id !== "string" || typeof object.kind !== "string") {
        throw new Error("Invalid SYN object");
      }
      if (!Number.isFinite(object.x) || !Number.isFinite(object.y) || !Number.isFinite(object.width) || !Number.isFinite(object.height)) {
        throw new Error("Invalid SYN object geometry");
      }
      if (object.props?.src && !String(object.props.src).startsWith("data:image/")) {
        throw new Error("SYN media must use embedded image data");
      }
    }
    for (const interaction of scene.interactions) {
      if (!interaction?.event?.type || !interaction?.event?.target || !Array.isArray(interaction.actions)) {
        throw new Error("Invalid SYN interaction");
      }
      if (interaction.event.type === "click" && !scene.objects.some(object => object.id === interaction.event.target)) {
        throw new Error("SYN interaction target not found");
      }
      for (const action of interaction.actions) {
        if (!action?.type || !ACTIONS.has(action.type)) {
          throw new Error("Unsupported SYN action");
        }
        if (action.type === "scene.goto" && !document.scenes.some(target => target.id === action.target)) {
          throw new Error("SYN action target not found");
        }
        if (["object.show", "object.hide", "object.setText"].includes(action.type) && !scene.objects.some(object => object.id === action.target)) {
          throw new Error("SYN action target not found");
        }
      }
    }
  }
  return document;
}

export function createRuntimeState(document) {
  validateRuntimeDocument(document);
  return {
    document,
    sceneIndex: 0,
    visible: new Map(),
    text: new Map()
  };
}

export function currentScene(state) {
  return state.document.scenes[state.sceneIndex] ?? null;
}

export function findObject(state, id) {
  return currentScene(state)?.objects.find(object => object.id === id) ?? null;
}

export function applyAction(state, action) {
  switch (action.type) {
    case "scene.next":
      state.sceneIndex = Math.min(state.sceneIndex + 1, state.document.scenes.length - 1);
      return true;
    case "scene.goto": {
      const index = state.document.scenes.findIndex(scene => scene.id === action.target);
      if (index < 0) return false;
      state.sceneIndex = index;
      return true;
    }
    case "object.show":
      state.visible.set(action.target, true);
      return true;
    case "object.hide":
      state.visible.set(action.target, false);
      return true;
    case "object.setText":
      state.text.set(action.target, String(action.value ?? ""));
      return true;
    default:
      return false;
  }
}

export function dispatchEvent(state, event) {
  const scene = currentScene(state);
  if (!scene) return false;
  let handled = false;
  for (const interaction of scene.interactions) {
    if (interaction.event.type !== event.type || interaction.event.target !== event.target) continue;
    for (const action of interaction.actions) handled = applyAction(state, action) || handled;
  }
  return handled;
}

export function renderScene(container, state, { onEvent } = {}) {
  const scene = currentScene(state);
  if (!scene) {
    container.replaceChildren();
    return;
  }

  container.replaceChildren();
  const doc = container.ownerDocument || document;
  const fragment = doc.createDocumentFragment();

  for (const object of scene.objects) {
    const element = doc.createElement(object.kind === "button" ? "button" : "div");
    element.className = "syn-runtime-object";
    element.dataset.id = object.id;
    element.dataset.kind = object.kind;
    element.textContent = state.text.get(object.id) ?? object.props?.text ?? object.label ?? object.kind;
    element.style.left = (object.x ?? 80) + "px";
    element.style.top = (object.y ?? 80) + "px";
    element.style.width = (object.width ?? 180) + "px";
    element.style.minHeight = (object.height ?? 60) + "px";
    if (object.props?.src?.startsWith("data:image/")) element.style.backgroundImage = `url("${object.props.src}")`;
    if (state.visible.get(object.id) === false) element.hidden = true;

    if (object.kind === "button") {
      element.type = "button";
      element.addEventListener("click", () => onEvent?.({ type: "click", target: object.id }));
    }
    fragment.appendChild(element);
  }

  container.appendChild(fragment);
}
