import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import path from "node:path";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const files = [
  "src/document.js",
  "src/runtime.js",
  "studio/app.js",
  "runtime/runtime.js"
];

test("browser and core JavaScript files parse successfully", () => {
  for (const file of files) {
    assert.doesNotThrow(
      () => execFileSync(process.execPath, ["--check", path.join(root, file)], { stdio: "pipe" }),
      file
    );
  }
});
