import { createRuntimeState, currentScene, dispatchEvent, renderScene } from "../src/runtime.js";

const input = document.querySelector("#file");
const stage = document.querySelector("#stage");
const empty = document.querySelector("#empty");
const title = document.querySelector("#title");
const sceneName = document.querySelector("#sceneName");
const sceneCount = document.querySelector("#sceneCount");
const error = document.querySelector("#error");

input.addEventListener("change", () => {
  const file = input.files && input.files[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => {
    try {
      const state = createRuntimeState(JSON.parse(reader.result));
      empty.hidden = true;
      error.hidden = true;
      title.textContent = state.document.meta.title;
      const mount = () => {
        const scene = currentScene(state);
        sceneName.textContent = scene ? scene.name : "No scene";
        sceneCount.textContent = "Scene " + (state.sceneIndex + 1) + " of " + state.document.scenes.length;
        renderScene(stage, state, { onEvent: event => { dispatchEvent(state, event); mount(); } });
      };
      mount();
    } catch (err) {
      stage.replaceChildren();
      empty.hidden = false;
      error.hidden = false;
      error.textContent = err instanceof Error ? err.message : "Unable to open SYN document.";
    }
  };
  reader.readAsText(file);
});
