// Compatibility service worker for the pre-Rust SYN Studio deployment.
//
// IMPORTANT:
// - This worker deliberately has NO fetch handler.
// - It exists only so browsers that still have the old cache-first worker
//   can replace it, clear its caches, and immediately take control.
// - Keeping the worker installed is intentional: otherwise a page that
//   registers it on load would simply reinstall it on every refresh.

self.addEventListener("install", event => {
  event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", event => {
  event.waitUntil((async () => {
    const keys = await caches.keys();
    await Promise.all(keys.map(key => caches.delete(key)));
    await self.clients.claim();

    const clients = await self.clients.matchAll({
      type: "window",
      includeUncontrolled: true
    });

    for (const client of clients) {
      try {
        await client.navigate(client.url);
      } catch (_) {
        // A client can disappear during activation. Nothing else is required.
      }
    }
  })());
});
