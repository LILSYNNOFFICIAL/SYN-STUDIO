import { randomUUID } from "node:crypto";

export function createSynDocument({ title = "Untitled SYN", id = randomUUID() } = {}) {
  return {
    syn: "0.1",
    type: "document",
    meta: {
      id,
      title
    },
    scenes: []
  };
}

export function serializeSynDocument(document) {
  return JSON.stringify(document, null, 2) + "\n";
}
