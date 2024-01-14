// Definieren Sie die Cache-Namen
const CACHE_NAME = 'meine-pwa-cache-v1';
const urlsToCache = [
  '/',
  '/pkg/visualtimer.js',
  '/pkg/visualtimer.css',
  '/pkg/visualtimer.wasm',
  // Fügen Sie weitere URLs hinzu, die Sie cachen möchten
];

// Installationsereignis
self.addEventListener('install', function(event) {
  // Führen Sie Schritte aus, um Ressourcen zu cachen
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(function(cache) {
        console.log('Geöffneter Cache');
        return cache.addAll(urlsToCache);
      })
  );
});

// Anfragen abfangen und aus dem Cache bedienen
self.addEventListener('fetch', function(event) {
  event.respondWith(
    caches.match(event.request)
      .then(function(response) {
        // Cache getroffen - die Antwort zurückgeben
        if (response) {
          return response;
        }
        return fetch(event.request);
      }
    )
  );
});

// Aktualisierung des Service Workers und Löschen alter Caches
self.addEventListener('activate', function(event) {
  var cacheWhitelist = ['meine-pwa-cache-v1'];

  event.waitUntil(
    caches.keys().then(function(cacheNames) {
      return Promise.all(
        cacheNames.map(function(cacheName) {
          if (cacheWhitelist.indexOf(cacheName) === -1) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
});
