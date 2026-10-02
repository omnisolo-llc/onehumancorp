# Observing the actual launch receipt across navigation

The legacy setup UI navigates after it consumes the real launch response. In the
57b hosted run, both maintained manual-completion journeys lost that response
body through Chromium's `Network.getResponseBody`, even though Playwright began
reading it as soon as the response arrived.

The launch-only test observer uses a separate passive CDP Network session. It
enables tracking and awaits `Network.configureDurableMessages` before the UI
action. The installed Playwright protocol declares this command as retaining
response bodies outside the renderer across process navigation. Its buffer is
bounded to 2 MiB total and 256 KiB per resource; request bodies are bounded to
64 KiB. The helper matches one exact same-origin POST path and request ID, then
reads that actual body after `loadingFinished`. Duplicate requests, redirects,
failed loads, missing/oversized bodies, unsupported protocol commands and timeout
are visible failures. Base64 bodies are validated and decoded as strict UTF-8.
Listeners and the session are removed on completion, error and early disposal.

This observer sends no HTTP request, injects no application code, changes no
headers, intercepts no traffic, and never repeats the mutation. Receipt assertions
still require the exact prepared/launched identity and protected persisted state.
The launch request's preparation ID and expected-owner headers are also checked.

Protocol-boundary unit tests use recorded events and cannot establish Chromium
support or product success. The maintained hosted browser journeys must pass on
the actual runner before this issue is closed. The other finite setup responses
continue to use the existing Playwright observation path.

Reference: [Chrome DevTools Network protocol](https://chromedevtools.github.io/devtools-protocol/tot/Network/#method-configureDurableMessages),
also declared by the repository's installed `playwright-core/types/protocol.d.ts`.
