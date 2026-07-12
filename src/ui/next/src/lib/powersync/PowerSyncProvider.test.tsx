import React from 'react';
import { act, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import { getPowerSyncDB } from './db';
import { isPowerSyncSupportedForLocation, PowerSyncProvider } from './PowerSyncProvider';

vi.mock('./db', () => ({
  getPowerSyncDB: vi.fn(),
}));

const getPowerSyncDBMock = vi.mocked(getPowerSyncDB);

function databaseMock(overrides: Record<string, unknown> = {}) {
  return {
    init: vi.fn().mockResolvedValue(undefined),
    connect: vi.fn().mockResolvedValue(undefined),
    disconnect: vi.fn().mockResolvedValue(undefined),
    close: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  };
}

beforeEach(() => {
  getPowerSyncDBMock.mockReset();
  Object.defineProperty(window, 'isSecureContext', {
    configurable: true,
    value: true,
  });
});

afterEach(() => {
  vi.restoreAllMocks();
});

test('allows PowerSync on secure browser contexts', () => {
  expect(isPowerSyncSupportedForLocation(true, '172.17.197.51')).toBe(true);
});

test('allows PowerSync on localhost even when served over http', () => {
  expect(isPowerSyncSupportedForLocation(false, 'localhost')).toBe(true);
  expect(isPowerSyncSupportedForLocation(false, '127.0.0.1')).toBe(true);
});

test('disables PowerSync on insecure IP-hosted pages', () => {
  expect(isPowerSyncSupportedForLocation(false, '172.17.197.51')).toBe(false);
});

test('handles a rejected connection as an unsupported fallback state', async () => {
  const connectionError = new Error('connection failed');
  const database = databaseMock({ connect: vi.fn().mockRejectedValue(connectionError) });
  getPowerSyncDBMock.mockResolvedValue(database as any);
  const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

  render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );

  expect(screen.getByText('Stable loading state')).toBeInTheDocument();
  await waitFor(() => expect(screen.getByText('API fallback')).toBeInTheDocument());
  expect(consoleError).toHaveBeenCalledWith(connectionError);
});

test('closes a database that resolves after the provider unmounts', async () => {
  let resolveDatabase!: (database: any) => void;
  const database = databaseMock();
  getPowerSyncDBMock.mockReturnValue(new Promise((resolve) => {
    resolveDatabase = resolve;
  }));

  const { unmount } = render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );
  unmount();

  await act(async () => {
    resolveDatabase(database);
    await Promise.resolve();
    await Promise.resolve();
  });

  expect(database.disconnect).toHaveBeenCalledOnce();
  expect(database.close).toHaveBeenCalledOnce();
  expect(database.init).not.toHaveBeenCalled();
  expect(database.connect).not.toHaveBeenCalled();
});
