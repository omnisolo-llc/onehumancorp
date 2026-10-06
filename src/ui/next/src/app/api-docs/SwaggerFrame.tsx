'use client';

import { useCallback, useEffect, useRef, useState } from 'react';

export default function SwaggerFrame({ spec }: { spec: Record<string, unknown> }) {
  const frame = useRef<HTMLIFrameElement>(null);
  const timer = useRef<number | undefined>(undefined);
  const [height, setHeight] = useState(600);
  const [ready, setReady] = useState(false);
  const [failed, setFailed] = useState(false);
  const initialize = useCallback(() => {
    setReady(false);
    setFailed(false);
    window.clearTimeout(timer.current);
    timer.current = window.setTimeout(() => setFailed(true), 15_000);
    frame.current?.contentWindow?.postMessage({
      type: 'ohc-api-docs:init', spec,
      dark: document.documentElement.classList.contains('dark'),
    }, window.location.origin);
  }, [spec]);

  useEffect(() => {
    const receive = (event: MessageEvent) => {
      if (event.origin !== window.location.origin || event.source !== frame.current?.contentWindow) return;
      const data = event.data;
      if (!data || typeof data !== 'object') return;
      if (data.type === 'ohc-api-docs:ready') {
        window.clearTimeout(timer.current);
        setReady(true);
        setFailed(false);
      } else if (data.type === 'ohc-api-docs:error') {
        window.clearTimeout(timer.current);
        setFailed(true);
      } else if (data.type === 'ohc-api-docs:resize' && Number.isFinite(data.height) && data.height > 0 && data.height <= 10_000_000) {
        setHeight(Math.max(600, Math.ceil(data.height)));
      }
    };
    window.addEventListener('message', receive);
    const theme = new MutationObserver(() => frame.current?.contentWindow?.postMessage({
      type: 'ohc-api-docs:theme', dark: document.documentElement.classList.contains('dark'),
    }, window.location.origin));
    theme.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
    return () => {
      window.clearTimeout(timer.current);
      window.removeEventListener('message', receive);
      theme.disconnect();
      // React removes the iframe itself, retiring the bundle's entire realm.
      // The official distribution does not expose its internal React root.
    };
  }, []);

  useEffect(() => { initialize(); }, [initialize]);

  return <>
    {failed && <p role="alert">Failed to load interactive API documentation. Reload this page to try again.</p>}
    <iframe ref={frame} src="/api-docs/viewer" title="Interactive API documentation"
      data-ohc-api-docs-viewer="true" aria-busy={!ready && !failed}
      onLoad={initialize} onError={() => setFailed(true)}
      className="w-full border-0" style={{ height }} />
  </>;
}
