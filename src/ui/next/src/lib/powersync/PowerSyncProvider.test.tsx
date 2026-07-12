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

test('keeps local content when the background connection rejects', async () => {
  let rejectConnection!: (error: Error) => void;
  const database = databaseMock({
    connect: vi.fn().mockReturnValue(new Promise((_, reject) => {
      rejectConnection = reject;
    })),
  });
  getPowerSyncDBMock.mockResolvedValue(database as any);
  const consoleWarn = vi.spyOn(console, 'warn').mockImplementation(() => {});
  const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

  render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );

  expect(screen.getByText('Stable loading state')).toBeInTheDocument();
  await waitFor(() => expect(screen.getByText('Inbox content')).toBeInTheDocument());

  await act(async () => {
    rejectConnection(new Error('token=credential-bearing-secret'));
    await Promise.resolve();
  });

  expect(screen.getByText('Inbox content')).toBeInTheDocument();
  expect(screen.queryByText('API fallback')).not.toBeInTheDocument();
  expect(consoleWarn).toHaveBeenCalledOnce();
  expect(consoleWarn).toHaveBeenCalledWith('PowerSync background sync connection failed; local data remains available.');
  expect(consoleError).not.toHaveBeenCalled();
});

test('does not initialize or dispose a shared database that resolves after unmount', async () => {
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

  expect(database.disconnect).not.toHaveBeenCalled();
  expect(database.close).not.toHaveBeenCalled();
  expect(database.init).not.toHaveBeenCalled();
  expect(database.connect).not.toHaveBeenCalled();
});

test('keeps the cached database usable after a connection failure and remount', async () => {
  let closed = false;
  const connectionError = new Error('connection failed');
  const database = databaseMock({
    init: vi.fn().mockImplementation(async () => {
      if (closed) throw new Error('database is closed');
    }),
    connect: vi.fn()
      .mockRejectedValueOnce(connectionError)
      .mockImplementation(async () => {
        if (closed) throw new Error('database is closed');
      }),
    close: vi.fn().mockImplementation(async () => {
      closed = true;
    }),
  });
  getPowerSyncDBMock.mockResolvedValue(database as any);
  vi.spyOn(console, 'warn').mockImplementation(() => {});

  const firstRender = render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );
  await waitFor(() => expect(screen.getByText('Inbox content')).toBeInTheDocument());
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
  });
  firstRender.unmount();

  render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );

  await waitFor(() => expect(screen.getByText('Inbox content')).toBeInTheDocument());
  expect(database.disconnect).not.toHaveBeenCalled();
  expect(database.close).not.toHaveBeenCalled();
});

test('suppresses a pending background connection rejection after unmount', async () => {
  let rejectConnection!: (error: Error) => void;
  const database = databaseMock({
    connect: vi.fn().mockReturnValue(new Promise((_, reject) => {
      rejectConnection = reject;
    })),
  });
  getPowerSyncDBMock.mockResolvedValue(database as any);
  const consoleWarn = vi.spyOn(console, 'warn').mockImplementation(() => {});
  const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

  const { unmount } = render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>} unsupportedFallback={<div>API fallback</div>}>
      <div>Inbox content</div>
    </PowerSyncProvider>,
  );
  await waitFor(() => expect(screen.getByText('Inbox content')).toBeInTheDocument());
  unmount();

  await act(async () => {
    rejectConnection(new Error('token=credential-bearing-secret'));
    await Promise.resolve();
  });

  expect(consoleWarn).not.toHaveBeenCalled();
  expect(consoleError).not.toHaveBeenCalled();
  expect(database.disconnect).not.toHaveBeenCalled();
  expect(database.close).not.toHaveBeenCalled();
});

test('does not dispose the cached database between successful provider mounts', async () => {
  let closed = false;
  const database = databaseMock({
    init: vi.fn().mockImplementation(async () => {
      if (closed) throw new Error('database is closed');
    }),
    connect: vi.fn().mockImplementation(async () => {
      if (closed) throw new Error('database is closed');
    }),
    close: vi.fn().mockImplementation(async () => {
      closed = true;
    }),
  });
  getPowerSyncDBMock.mockResolvedValue(database as any);

  const firstRender = render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>}>
      <div>First inbox mount</div>
    </PowerSyncProvider>,
  );
  await waitFor(() => expect(screen.getByText('First inbox mount')).toBeInTheDocument());
  firstRender.unmount();

  render(
    <PowerSyncProvider fallback={<div>Stable loading state</div>}>
      <div>Second inbox mount</div>
    </PowerSyncProvider>,
  );

  await waitFor(() => expect(screen.getByText('Second inbox mount')).toBeInTheDocument());
  expect(database.connect).toHaveBeenCalledTimes(2);
  expect(database.disconnect).not.toHaveBeenCalled();
  expect(database.close).not.toHaveBeenCalled();
});
