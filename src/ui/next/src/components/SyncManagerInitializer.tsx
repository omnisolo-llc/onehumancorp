"use client";

import { useEffect } from 'react';
import { notifyQueueIdentityChange } from '../lib/sync/queueIdentity';
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
    // Ensure the SyncManager is instantiated on mount
    SyncManager.getInstance();
  }, []);

  return null;
}
