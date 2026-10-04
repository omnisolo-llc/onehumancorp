"use client";

import { useEffect } from 'react';
import { notifyQueueIdentityChange, QUEUE_IDENTITY_EPOCH_KEY } from '../lib/sync/queueIdentity';
import { SyncManager } from '../lib/sync/SyncManager';

export function SyncManagerInitializer() {
  useEffect(() => {
    // Ordinary app/preview mounts are not authentication changes. OIDC marks
    // its successful return explicitly; this signal never supplies identity.
    const location = new URL(window.location.href);
    if (location.searchParams.get('ohc_auth_complete') === '1') {
      location.searchParams.delete('ohc_auth_complete');
      window.history.replaceState(window.history.state, '', location.pathname + location.search + location.hash);
      notifyQueueIdentityChange();
    }
    const manager = SyncManager.getInstance();
    const resume = () => { void manager.resumeFieldCompletionWork(); };
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) resume(); };
    resume();
    window.addEventListener('omnisolo_auth_changed', resume);
    window.addEventListener('storage', storage);
    window.addEventListener('pageshow', resume);
    return () => {
      window.removeEventListener('omnisolo_auth_changed', resume);
      window.removeEventListener('storage', storage);
      window.removeEventListener('pageshow', resume);
    };
  }, []);

  return null;
}
