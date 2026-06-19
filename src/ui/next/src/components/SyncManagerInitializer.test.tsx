import React from 'react';
import { render } from '@testing-library/react';
import { SyncManagerInitializer } from './SyncManagerInitializer';
import { SyncManager } from '../lib/sync/SyncManager';
import { describe, it, expect, vi } from 'vitest';

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
