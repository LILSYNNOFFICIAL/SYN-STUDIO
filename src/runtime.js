import { parseSynDocument } from "./document.js";

const ACTIONS = new Set([
  "scene.next",
  "scene.goto",
  "object.show",
  "object.hide",
  "object.setText",
  "link.openUrl",
  "link.openSyn"
]);

export function validateRuntimeDocument(document) {
  parseSynDocument(JSON.stringify(document));
  for (const scene of document.scenes) {
    for (const object of scene.objects) {
      if (!Number.isFinite(object.x) || !Number.isFinite(object.y) || !Number.isFinite(object.width) || !Number.isFinite(object.height)) {
        throw new Error("Invalid SYN object geometry");
      }
      if (object.props?.src && !String(object.props.src).startsWith("data:image/")) {
        throw new Error("SYN media must use embedded image data");
      }
      if (object.links && !Array.isArray(object.links)) throw new Error("Invalid SYN object links");
    }
    for (const interaction of scene.interactions) {
      if (!interaction?.event?.type || !interaction?.event?.target || !Array.isArray(interaction.actions)) {
        throw new Error("Invalid SYN interaction");
      }
      if (interaction.event.type === "click" && !scene.objects.some(object => object.id === interaction.event.target)) {
        throw new Error("SYN interaction target not found");
      }
      for (const action of interaction.actions) {
        if (!action?.type || !ACTIONS.has(action.type)) throw new Error("Unsupported SYN action");
        if (action.type === "scene.goto" && !document.scenes.some(target => target.id === action.target)) {
          throw new Error("SYN action target not found");
        }
        if (["object.show","object.hide","object.setText"].includes(action.type) &&
            !scene.objects.some(object => object.id === action.target)) {
          throw new Error("SYN action target not found");
        }
        if (action.type === "link.openUrl" && (!/^https?:\/\//i.test(String(action.url || "")))) {
          throw new Error("SYN URL links must use http or https");
        }
        if (action.type === "link.openSyn" && typeof action.target !== "string") {
          throw new Error("SYN document link target is required");
        }
      }
    }
  }
  return document;
}

export function createRuntimeState(document) {
  validateRuntimeDocument(document);
  return { document, sceneIndex: 0, visible: new Map(), text: new Map() };
}

export function responsiveScale(documentWidth, documentHeight, viewportWidth, viewportHeight) {
  if (![documentWidth, documentHeight, viewportWidth, viewportHeight].every(Number.isFinite) || documentWidth <= 0 || documentHeight <= 0 || viewportWidth <= 0 || viewportHeight <= 0) return 1;
  return Math.min(viewportWidth / documentWidth, viewportHeight / documentHeight);
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
      state.sceneIndex = Math.min(state.sceneIndex + 1, Math.max(0, state.document.scenes.length - 1));
      return true;
    case "scene.goto": {
      const index = state.document.scenes.findIndex(scene => scene.id === action.target);
      if (index < 0) return false;
      state.sceneIndex = index;
      return true;
    }
    case "object.show": state.visible.set(action.target, true); return true;
    case "object.hide": state.visible.set(action.target, false); return true;
    case "object.setText": state.text.set(action.target, String(action.value ?? "")); return true;
    case "link.openUrl": window.open(action.url, "_blank", "noopener,noreferrer"); return true;
    case "link.openSyn": window.location.href = action.target; return true;
    default: return false;
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

function applyStyles(element, object) {
  const s = object.styles || {};
  if (s.fontFamily) element.style.fontFamily = s.fontFamily;
  if (s.fontSize) element.style.fontSize = s.fontSize + "px";
  if (s.fontWeight) element.style.fontWeight = s.fontWeight;
  if (s.fontStyle) element.style.fontStyle = s.fontStyle;
  if (s.color) element.style.color = s.color;
  if (s.background) element.style.background = s.background;
  if (s.borderColor) element.style.borderColor = s.borderColor;
  if (s.borderWidth != null) element.style.borderWidth = s.borderWidth + "px";
  if (s.borderRadius != null) element.style.borderRadius = s.borderRadius + "px";
  if (s.opacity != null) element.style.opacity = s.opacity;
  if (s.letterSpacing != null) element.style.letterSpacing = s.letterSpacing + "px";
  if (s.lineHeight != null) element.style.lineHeight = s.lineHeight;
  if (s.textAlign) element.style.textAlign = s.textAlign;
  if (s.boxShadow) element.style.boxShadow = s.boxShadow;
}

export function renderScene(container, state, { onEvent } = {}) {
  const scene = currentScene(state);
  if (!scene) { container.replaceChildren(); return; }
  container.replaceChildren();
  const doc = container.ownerDocument || document;
  const fragment = doc.createDocumentFragment();
  const logical = state.document.viewport || { width: 1120, height: 640 };
  const scale = Math.min(1, responsiveScale(logical.width, logical.height, Math.max(1, container.clientWidth - 2), Math.max(1, container.clientHeight - 2)));
  for (const object of scene.objects) {
    const element = doc.createElement(object.kind === "button" ? "button" : "div");
    element.className = "syn-runtime-object";
    element.dataset.id = object.id;
    element.dataset.kind = object.kind;
    element.textContent = state.text.get(object.id) ?? object.props?.text ?? object.label ?? object.kind;
    element.style.left = Math.round((object.x ?? 80) * scale) + "px";
    element.style.top = Math.round((object.y ?? 80) * scale) + "px";
    element.style.width = Math.max(20, Math.round((object.width ?? 180) * scale)) + "px";
    element.style.height = Math.max(20, Math.round((object.height ?? 60) * scale)) + "px";
    applyStyles(element, object);
    if (object.styles?.fontSize) element.style.fontSize = Math.max(8, object.styles.fontSize * scale) + "px";
    if (object.props?.src?.startsWith("data:image/")) {
      element.style.backgroundImage = `url("${object.props.src}")`;
      element.style.backgroundSize = "cover";
      element.style.backgroundPosition = "center";
    }
    if (state.visible.get(object.id) === false) element.hidden = true;
    if (object.kind === "button") {
      element.type = "button";
      element.addEventListener("click", () => onEvent?.({ type: "click", target: object.id }));
    }
    fragment.appendChild(element);
  }
  container.appendChild(fragment);
}
