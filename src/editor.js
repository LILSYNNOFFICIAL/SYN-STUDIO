function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

export function moveObject(object, x, y, viewport) {
  const maxX = Math.max(0, viewport.width - object.width);
  const maxY = Math.max(0, viewport.height - object.height);
  object.x = Math.round(clamp(Number(x) || 0, 0, maxX));
  object.y = Math.round(clamp(Number(y) || 0, 0, maxY));
  return object;
}

export function resizeObject(object, width, height, viewport, minSize = 20) {
  const maxWidth = Math.max(minSize, viewport.width - object.x);
  const maxHeight = Math.max(minSize, viewport.height - object.y);
  object.width = Math.round(clamp(Number(width) || minSize, minSize, maxWidth));
  object.height = Math.round(clamp(Number(height) || minSize, minSize, maxHeight));
  return object;
}

export function duplicateObject(scene, objectId) {
  const source = scene.objects.find(object => object.id === objectId);
  if (!source) return null;
  const copy = structuredClone(source);
  copy.id = globalThis.crypto?.randomUUID?.() || "syn-copy-" + Date.now().toString(36) + "-" + Math.random().toString(36).slice(2);
  copy.label = (source.label || "Object") + " Copy";
  copy.x += 24;
  copy.y += 24;
  const index = scene.objects.findIndex(object => object.id === objectId);
  scene.objects.splice(index + 1, 0, copy);
  return copy;
}

export function reorderObject(scene, objectId, direction) {
  const index = scene.objects.findIndex(object => object.id === objectId);
  if (index < 0) return false;
  const next = direction === "front" ? scene.objects.length - 1 : 0;
  if (index === next) return false;
  const [object] = scene.objects.splice(index, 1);
  scene.objects.splice(next, 0, object);
  return true;
}

export function hitTest(scene, x, y) {
  for (let i = scene.objects.length - 1; i >= 0; i -= 1) {
    const object = scene.objects[i];
    if (object.hidden || object.locked) continue;
    if (x >= object.x && y >= object.y && x <= object.x + object.width && y <= object.y + object.height) return object;
  }
  return null;
}
