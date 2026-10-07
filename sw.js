const CACHE = "syn-studio-v1";
const CORE = [
  "./",
  "./index.html",
  "./studio/",
  "./studio/index.html",
  "./studio/styles.css",
  "./studio/app.js",
  "./runtime/",
  "./runtime/index.html",
  "./runtime/runtime.css",
  "./runtime/runtime.js",
  "./src/document.js",
  "./src/runtime.js"
];

self.addEventListener("install", event => {
  event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(CORE)));
  self.skipWaiting();
});

self.addEventListener("activate", event => {
  event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key)))));
  self.clients.claim();
});

self.addEventListener("fetch", event => {
  if (event.request.method !== "GET") return;
  event.respondWith(caches.match(event.request).then(cached => cached || fetch(event.request)));
});
