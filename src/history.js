export function createHistory(initial, { limit = 100 } = {}) {
  const entries = [structuredClone(initial)];
  let cursor = 0;
  const max = Math.max(1, limit);

  function push(value) {
    entries.splice(cursor + 1);
    entries.push(structuredClone(value));
    if (entries.length > max) entries.shift();
    cursor = entries.length - 1;
  }

  return {
    push,
    undo() {
      if (cursor === 0) return null;
      cursor -= 1;
      return structuredClone(entries[cursor]);
    },
    redo() {
      if (cursor >= entries.length - 1) return null;
      cursor += 1;
      return structuredClone(entries[cursor]);
    },
    canUndo: () => cursor > 0,
    canRedo: () => cursor < entries.length - 1
  };
}
