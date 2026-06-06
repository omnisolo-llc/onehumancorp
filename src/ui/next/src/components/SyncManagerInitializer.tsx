"use client";

import { useEffect } from 'react';
import { SyncManager } from '../lib/sync/SyncManager';

export function SyncManagerInitializer() {
  useEffect(() => {
    // Ensure the SyncManager is instantiated on mount
    SyncManager.getInstance();
  }, []);

  return null;
}
