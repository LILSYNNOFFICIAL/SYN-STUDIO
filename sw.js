// One-time cleanup for the pre-Rust SYN Studio service worker.
// The Rust/Dioxus build no longer uses the legacy cached application.
self.addEventListener("install", () => self.skipWaiting());

self.addEventListener("activate", event => {
  event.waitUntil((async () => {
    const keys = await caches.keys();
    await Promise.all(keys.map(key => caches.delete(key)));
    await self.registration.unregister();
    const clients = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
    for (const client of clients) client.navigate(client.url);
  })());
});