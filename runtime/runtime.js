import { createRuntimeState } from "../src/runtime.js";

const fileInput = document.querySelector("#file");
console.log("SYN Runtime ready", Boolean(fileInput), createRuntimeState);