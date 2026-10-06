// No specification or credentials are interpolated into this document.
// The parent passes the authenticated JSON through a source/origin-checked message.
export const VIEWER_SCRIPT = String.raw`(() => {
  let ui;
  const notify = (data) => parent.postMessage(data, location.origin);
  const resize = () => {
    const container = document.getElementById('swagger-ui');
    notify({ type: 'ohc-api-docs:resize', height: Math.ceil(container.getBoundingClientRect().bottom + window.scrollY + 16) });
  };
  const theme = (dark) => document.documentElement.classList.toggle('dark', dark === true);
  const observer = new ResizeObserver(resize);
  observer.observe(document.getElementById('swagger-ui'));
  window.addEventListener('message', (event) => {
    if (event.source !== parent || event.origin !== location.origin || !event.data || typeof event.data !== 'object') return;
    const data = event.data;
    if (data.type === 'ohc-api-docs:theme') { theme(data.dark); return; }
    if (data.type !== 'ohc-api-docs:init' || !data.spec || typeof data.spec !== 'object' || Array.isArray(data.spec)) return;
    theme(data.dark);
    try {
      if (ui) {
        ui.specActions.updateSpec(JSON.stringify(data.spec));
        return;
      }
      ui = SwaggerUIBundle({
        dom_id: '#swagger-ui', spec: data.spec,
        validatorUrl: null, queryConfigEnabled: false,
        oauth2RedirectUrl: location.origin + '/vendor/swagger-ui/dist/oauth2-redirect.html',
        onComplete: () => { notify({ type: 'ohc-api-docs:ready' }); resize(); },
      });
    } catch {
      document.getElementById('viewer-error').hidden = false;
      notify({ type: 'ohc-api-docs:error' });
    }
  });
  window.addEventListener('pagehide', () => observer.disconnect(), { once: true });
})();`;

export const VIEWER_HTML = `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>Interactive API documentation</title>
<link rel="stylesheet" href="/vendor/swagger-ui/dist/swagger-ui.css">
<style>html, body { margin: 0; padding: 0; background: transparent; font-family: sans-serif; }
html { color-scheme: light; }
html.dark { color-scheme: dark; }
html.dark .swagger-ui { filter: invert(.88) hue-rotate(180deg); }
html.dark .swagger-ui img { filter: invert(1) hue-rotate(180deg); }

        .swagger-ui { background: transparent; border-radius: 12px; padding: 12px; width: 100%; box-sizing: border-box; max-width: 100vw; overflow-x: hidden; }
        @media (min-width: 640px) { .swagger-ui { padding: 24px; } }
        .swagger-ui .wrapper { width: 100%; max-width: 100vw; overflow-x: hidden; padding: 0 10px; box-sizing: border-box; }
        .swagger-ui .opblock-body pre { white-space: pre-wrap; word-wrap: break-word; overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui table { display: block; overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .markdown p { word-break: break-word; box-sizing: border-box; }
        .swagger-ui .info { margin: 20px 0; box-sizing: border-box; }
        .swagger-ui .scheme-container { background: transparent; padding: 10px 0; margin-bottom: 20px; border-radius: 12px; box-shadow: none; background: rgba(255, 255, 255, 0.4); backdrop-filter: blur(20px); box-shadow: 0 4px 12px rgba(0,0,0,0.05); border: 1px solid rgba(255,255,255,0.3); box-sizing: border-box; width: 100%; }
        .swagger-ui .responses-inner { overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .model-box { overflow-x: auto; max-width: 100%; box-sizing: border-box; }
        .swagger-ui .opblock-tag { font-size: 20px; padding: 10px; box-sizing: border-box; }
        .swagger-ui .opblock .opblock-summary { padding: 5px; box-sizing: border-box; }
        .swagger-ui .opblock .opblock-summary-method { min-width: 60px; font-size: 12px; }
        .swagger-ui .opblock .opblock-summary-path { font-size: 14px; max-width: calc(100vw - 120px); overflow-wrap: break-word; word-break: break-all; }
      </style></head>
<body><p id="viewer-error" role="alert" hidden>Failed to load interactive API documentation.</p><div id="swagger-ui"></div>
<script src="/vendor/swagger-ui/dist/swagger-ui-bundle.js"></script><script>${VIEWER_SCRIPT}</script></body></html>`;
