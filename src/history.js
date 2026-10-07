export function createHistory(initial, { limit = 100 } = {}) {
  const entries = [structuredClone(initial)];
  const future = [];
  const max = Math.max(2, limit);

  function push(value) {
    entries.push(structuredClone(value));
    future.length = 0;
    if (entries.length > max) entries.shift();
  }

  return {
    push,
    undo(currentValue) {
      if (entries.length <= 1) return null;
      if (currentValue !== undefined) future.unshift(structuredClone(currentValue));
      entries.pop();
      return structuredClone(entries[entries.length - 1]);
    },
    redo() {
      if (!future.length) return null;
      const next = future.shift();
      entries.push(structuredClone(next));
      if (entries.length > max) entries.shift();
      return structuredClone(next);
    },
    canUndo: () => entries.length > 1,
    canRedo: () => future.length > 0
  };
}
