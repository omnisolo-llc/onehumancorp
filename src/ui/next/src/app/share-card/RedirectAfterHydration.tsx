'use client';

import { useEffect, useState } from 'react';

const redirectedDocuments = new WeakSet<Document>();

export function redirectSharedDocument(document: Document, target: string, replace: (url: string) => void): void {
  if (redirectedDocuments.has(document)) return;
  // Mark before the side effect. StrictMode, hydration replay or a remount must
  // never schedule a second full document navigation, even if replace throws.
  redirectedDocuments.add(document);
  replace(target);
}

export default function RedirectAfterHydration({ targetUrl }: { targetUrl: string }) {
  const [failed, setFailed] = useState(false);
  useEffect(() => {
    try { redirectSharedDocument(document, targetUrl, url => window.location.replace(url)); }
    catch { setFailed(true); }
  }, [targetUrl]);

  return <div className="min-h-screen flex items-center justify-center bg-gray-50" aria-busy={!failed}>
    <p className="text-gray-600 font-medium">{failed ? 'Continue to ' : 'Redirecting to '}<a href={targetUrl} className="text-[#0071E3] hover:underline">{targetUrl}</a>{failed ? '.' : '…'}</p>
  </div>;
}
