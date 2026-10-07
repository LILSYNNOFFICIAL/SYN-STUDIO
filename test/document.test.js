import test from "node:test";
import assert from "node:assert/strict";
import { createSynDocument, serializeSynDocument } from "../src/document.js";

test("creates a minimal SYN document with a stable document identity", () => {
  const document = createSynDocument({ title: "Hello SYN" });

  assert.equal(document.syn, "0.1");
  assert.equal(document.type, "document");
  assert.equal(document.meta.title, "Hello SYN");
  assert.equal(typeof document.meta.id, "string");
  assert.match(document.meta.id, /^[0-9a-f-]{36}$/);
  assert.deepEqual(document.scenes, []);
});

test("serializes a SYN document as deterministic JSON", () => {
  const document = createSynDocument({ title: "Hello SYN", id: "00000000-0000-0000-0000-000000000001" });

  assert.equal(
    serializeSynDocument(document),
    JSON.stringify(document, null, 2) + "\n"
  );
});
