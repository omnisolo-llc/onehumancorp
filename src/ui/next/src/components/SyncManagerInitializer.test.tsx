import React from 'react';
import { render } from '@testing-library/react';
import { SyncManagerInitializer } from './SyncManagerInitializer';
import { SyncManager } from '../lib/sync/SyncManager';
import { afterEach, beforeEach, describe, it, expect, vi } from 'vitest';
import { QUEUE_IDENTITY_EPOCH_KEY } from '../lib/sync/queueIdentity';

beforeEach(() => { localStorage.clear(); window.history.replaceState(null, '', '/'); vi.clearAllMocks(); });
afterEach(() => { window.history.replaceState(null, '', '/'); });

vi.mock('../lib/sync/SyncManager', () => {
  return {
    SyncManager: {
      getInstance: vi.fn(),
    },
  };
});

describe('SyncManagerInitializer', () => {
  it('calls SyncManager.getInstance on mount', () => {
    render(<SyncManagerInitializer />);
    expect(SyncManager.getInstance).toHaveBeenCalled();
  });
});


it.each(['/onboarding/zero-click', '/builder?tenant=one&preview=true'])('mounting %s keeps a prepared parent and other tabs intact', (path) => {
  window.history.replaceState(null, '', path);
  localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'existing-login');
  const signal = vi.fn(); window.addEventListener('omnisolo_auth_changed', signal);
  try {
    const first = render(<SyncManagerInitializer />);
    expect(localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)).toBe('existing-login');
    first.unmount(); render(<SyncManagerInitializer />);
    expect(signal).not.toHaveBeenCalled();
  } finally { window.removeEventListener('omnisolo_auth_changed', signal); }
});

it('consumes an explicit OIDC completion once, preserves the return URL, and invalidates shared identity', () => {
  window.history.replaceState({ existing: 'router state' }, '', '/onboarding/zero-click?ohc_auth_complete=1&tab=review#summary');
  localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'previous-login');
  const signal = vi.fn(); window.addEventListener('omnisolo_auth_changed', signal);
  try {
    const first = render(<SyncManagerInitializer />);
    const epoch = localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
    expect(epoch).toBeTruthy(); expect(epoch).not.toBe('previous-login');
    expect(window.location.pathname + window.location.search + window.location.hash).toBe('/onboarding/zero-click?tab=review#summary');
    expect(window.history.state).toEqual({ existing: 'router state' });
    first.unmount(); render(<SyncManagerInitializer />);
    expect(localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)).toBe(epoch);
    expect(signal).toHaveBeenCalledTimes(1);
  } finally { window.removeEventListener('omnisolo_auth_changed', signal); }
});
