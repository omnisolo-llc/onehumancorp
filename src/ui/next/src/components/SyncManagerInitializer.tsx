"use client";

import { useEffect } from 'react';
import { notifyQueueIdentityChange } from '../lib/sync/queueIdentity';
import { SyncManager } from '../lib/sync/SyncManager';

export function SyncManagerInitializer() {
  useEffect(() => {
    // A fresh app document (including an OIDC return) invalidates old tab identities.
    notifyQueueIdentityChange();
    // Ensure the SyncManager is instantiated on mount
    SyncManager.getInstance();
  }, []);

  return null;
}
